/*
 * Copyright(c) 2013 Tim Ruehsen
 * Copyright(c) 2015 Free Software Foundation, Inc.
 *
 * This file is part of libwget.
 *
 * Libwget is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Libwget is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with libwget.  If not, see <http://www.gnu.org/licenses/>.
 *
 *
 * Highlevel HTTP functions
 *
 * Changelog
 * 21.01.2013  Tim Ruehsen  created
 *
 */

#if HAVE_CONFIG_H
# include <config.h>
#endif

#include <string.h>

#include <libwget.h>
#include "private.h"

wget_http_response_t *
wget_http_get (int first_key, ...)
{
  wget_vector_t *headers = wget_vector_create (8, 8, NULL);
  wget_iri_t *uri = NULL;
  wget_http_connection_t *conn = NULL, **connp = NULL;
  wget_http_request_t *req;
  wget_http_response_t *resp = NULL;
  wget_vector_t *challenges = NULL;
  wget_cookie_db_t *cookie_db = NULL;
  FILE *saveas_stream = NULL;
  int (*saveas_callback) (void *, const char *, size_t) = NULL;
  int saveas_fd = -1;
  int (*header_callback) (void *, wget_http_response_t *) = NULL;
  va_list args;
  const char *url = NULL, *url_encoding = NULL;
  const char *http_username = NULL, *http_password = NULL;
  int key, it, max_redirections = 5, redirection_level = 0;

  struct
  {
    unsigned int cookies_enabled:1, keep_header:1, free_uri:1;
  } bits =
      {
        .cookies_enabled = ! !wget_global_get_int (WGET_COOKIES_ENABLED)};

  va_start (args, first_key);
  for (key = first_key; key; key = va_arg (args, int))
    {
      switch (key)
        {
        case WGET_HTTP_URL:
          url = va_arg (args, const char *);
          break;
        case WGET_HTTP_URI:
          uri = va_arg (args, wget_iri_t *);
          break;
        case WGET_HTTP_URL_ENCODING:
          url_encoding = va_arg (args, const char *);
          break;
        case WGET_HTTP_HEADER_ADD:
          {
            wget_http_header_param_t param = {
              .name = va_arg (args, const char *),
              .value = va_arg (args, const char *)
            };
            wget_vector_add (headers, &param, sizeof (param));
            break;
          }
        case WGET_HTTP_CONNECTION_PTR:
          connp = va_arg (args, wget_http_connection_t **);
          if (connp)
            conn = *connp;
          break;
        case WGET_HTTP_RESPONSE_KEEPHEADER:
          bits.keep_header = va_arg (args, int);
          break;
        case WGET_HTTP_MAX_REDIRECTIONS:
          max_redirections = va_arg (args, int);
          break;
        case WGET_HTTP_BODY_SAVEAS_STREAM:
          saveas_stream = va_arg (args, FILE *);
          break;
        case WGET_HTTP_BODY_SAVEAS_FUNC:
          saveas_callback =
            va_arg (args, int (*)(void *, const char *, size_t));
          break;
        case WGET_HTTP_BODY_SAVEAS_FD:
          saveas_fd = va_arg (args, int);
          break;
        case WGET_HTTP_HEADER_FUNC:
          header_callback =
            va_arg (args, int (*)(void *, wget_http_response_t *));
          break;
        default:
          error_printf (_("Unknown option %d\n"), key);
          goto out;
        }
    }
  va_end (args);

  if (url && !uri)
    {
      uri = wget_iri_parse (url, url_encoding);
      bits.free_uri = 1;
    }

  if (!uri)
    {
      error_printf (_("Missing URL/URI\n"));
      goto out;
    }

  if (bits.cookies_enabled)
    cookie_db = (wget_cookie_db_t *) wget_global_get_ptr (WGET_COOKIE_DB);

  while (uri && redirection_level <= max_redirections)
    {
      // create a HTTP/1.1 GET request.
      // the only default header is 'Host: domain' (taken from uri)
      req = wget_http_create_request (uri, "GET");

      // add HTTP headers
      for (it = 0; it < wget_vector_size (headers); it++)
        {
          wget_http_add_header_param (req, wget_vector_get (headers, it));
        }

      if (challenges)
        {
          // There might be more than one challenge, we could select the most secure one.
          // For simplicity and testing we just take the first for now.
          // the following adds an Authorization: HTTP header
          wget_http_add_credentials (req, wget_vector_get (challenges, 0),
                                     http_username, http_password);
          wget_http_free_challenges (&challenges);
        }

      // use keep-alive if you want to send more requests on the same connection
      // http_add_header(req, "Connection", "keep-alive");

      // enrich the HTTP request with the uri-related cookies we have
      if (cookie_db)
        {
          const char *cookie_string;
          if ((cookie_string =
               wget_cookie_create_request_header (cookie_db, uri)))
            {
              wget_http_add_header (req, "Cookie", cookie_string);
              xfree (cookie_string);
            }
        }

      // open/reopen/reuse HTTP/HTTPS connection
      if (conn && !wget_strcmp (conn->esc_host, uri->host) &&
          conn->scheme == uri->scheme &&
          !wget_strcmp (conn->port, uri->resolv_port))
        {
          debug_printf ("reuse connection %s\n", conn->esc_host);
        }
      else
        {
          if (conn)
            {
              debug_printf ("close connection %s\n", conn->esc_host);
              wget_http_close (&conn);
            }
          if (wget_http_open (&conn, uri) == WGET_E_SUCCESS)
            debug_printf ("opened connection %s\n", conn->esc_host);
        }

      if (conn)
        {
          if (wget_http_send_request (conn, req) == 0)
            {
              if (saveas_stream)
                resp =
                  wget_http_get_response_stream (conn, header_callback,
                                                 saveas_stream,
                                                 WGET_HTTP_RESPONSE_KEEPHEADER);
              else if (saveas_callback)
                resp =
                  wget_http_get_response_func (conn, header_callback,
                                               saveas_callback, NULL,
                                               WGET_HTTP_RESPONSE_KEEPHEADER);
              else if (saveas_fd != -1)
                resp =
                  wget_http_get_response_fd (conn, header_callback, saveas_fd,
                                             WGET_HTTP_RESPONSE_KEEPHEADER);
              else
                resp =
                  wget_http_get_response (conn, header_callback, req,
                                          WGET_HTTP_RESPONSE_KEEPHEADER);
            }
        }

      wget_http_free_request (&req);

      if (!resp)
        goto out;

      // server doesn't support or want keep-alive
      if (!resp->keep_alive)
        wget_http_close (&conn);

      if (cookie_db)
        {
          // check and normalization of received cookies
          wget_cookie_normalize_cookies (uri, resp->cookies);

          // put cookies into cookie-store (also known as cookie-jar)
          wget_cookie_store_cookies (cookie_db, resp->cookies);
        }

      if (resp->code == 401 && !challenges)
        {      // Unauthorized
          if ((challenges = resp->challenges))
            {
              resp->challenges = NULL;
              wget_http_free_response (&resp);
              continue;    // try again with credentials
            }
          break;
        }

      // 304 Not Modified
      if (resp->code / 100 == 2 || resp->code / 100 >= 4 || resp->code == 304)
        break;      // final response

      if (resp->location)
        {
          char uri_sbuf[1024];
          wget_buffer_t uri_buf;

          // if relative location, convert to absolute
          wget_buffer_init (&uri_buf, uri_sbuf, sizeof (uri_sbuf));
          wget_iri_relative_to_abs (uri, resp->location,
                                    strlen (resp->location), &uri_buf);

          if (bits.free_uri)
            wget_iri_free (&uri);

          uri = wget_iri_parse (uri_buf.data, NULL);
          bits.free_uri = 1;

          wget_buffer_deinit (&uri_buf);

          redirection_level++;
          continue;
        }

      break;
    }


 out:
  if (connp)
    {
      *connp = conn;
    }
  else
    {
      wget_http_close (&conn);
    }

  wget_http_free_challenges (&challenges);

  //  wget_vector_clear_nofree(headers);
  wget_vector_free (&headers);

  if (bits.free_uri)
    wget_iri_free (&uri);

  return resp;
}
