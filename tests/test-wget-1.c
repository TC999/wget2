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
 * Testing Wget
 *
 * Changelog
 * 10.03.2013  Tim Ruehsen  created
 *
 */

#if HAVE_CONFIG_H
# include <config.h>
#endif

#include <stdlib.h>    // exit()
#include <string.h>    // strlen()
#include "libtest.h"

static const char *mainpage = "\
<html>\n\
<head>\n\
  <title>Main Page</title>\n\
</head>\n\
<body>\n\
  <p>\n\
    Some text and a link to a <a href=\"http://localhost:{{port}}/secondpage.html\">second page</a>.\n\
    Also, a <a href=\"http://localhost:{{port}}/nonexistent\">broken link</a>.\n\
  </p>\n\
</body>\n\
</html>\n";

static const char *secondpage = "\
<html>\n\
<head>\n\
  <title>Main Page</title>\n\
</head>\n\
<body>\n\
  <p>\n\
    Some text and a link to a <a href=\"http://localhost:{{port}}/thirdpage.html\">third page</a>.\n\
    Also, a <a href=\"http://localhost:{{port}}/nonexistent\">broken link</a>.\n\
  </p>\n\
</body>\n\
</html>\n";

static const char *thirdpage = "\
<html>\n\
<head>\n\
  <title>Main Page</title>\n\
</head>\n\
<body>\n\
  <p>\n\
    Some text and a link to a <a href=\"http://localhost:{{port}}/dummy.txt\">dummy text</a>.\n\
    Also, a <a href=\"http://localhost:{{port}}/againnonexistent\">broken link</a>.\n\
  </p>\n\
</body>\n\
</html>\n";

static const char *dummypage = "\
<html>\n\
<head>\n\
  <title>Main Page</title>\n\
</head>\n\
<body>\n\
  <p>\n\
    Don't care.\n\
  </p>\n\
</body>\n\
</html>\n";

int
main (void)
{
  wget_test_url_t urls[] = {
    {.name = "/index.html",
     .code = "200 Dontcare",
     .body = mainpage,
     .headers = {
        "Content-Type: text/html",
      }
    },
    {.name = "/secondpage.html",
     .code = "200 Dontcare",
     .body = secondpage,
     .headers = {
        "Content-Type: text/html",
      }
    },
    {.name = "/thirdpage.html",
     .code = "200 Dontcare",
     .body = thirdpage,
     .headers = {
        "Content-Type: text/html",
      }
    },
    {.name = "/dummy.txt",
     .code = "200 Dontcare",
     .body = "Don't care.",
     .headers = {
        "Content-Type: text/plain",
      },
    },
    {.name = "/dummy.html",
     .code = "200 Dontcare",
     .body = dummypage,
     .headers = {
        "Content-Type: text/plain",
        "Content-Disposition: attachment; filename=\"filename.html\"",
      }
    },
    {.name = "/dummy2.html",
     .code = "200 Dontcare",
     .body = dummypage,
     .headers = {
        "Content-Type: text/plain",
        "Content-Disposition: attachment; filename*=UTF-8''%66ile_fran%c3%A7ais.html",
      }
    }
  };

  // functions won't come back if an error occurs
  wget_test_start_server (WGET_TEST_RESPONSE_URLS, &urls, countof (urls), 0);

  // test-noop
  wget_test (WGET_TEST_REQUEST_URL, "index.html",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "index.html", urls[0].body},
                 {
                   NULL}}, 0);

  // test-nonexistent-quiet
  wget_test (WGET_TEST_OPTIONS, "--quiet",
             WGET_TEST_REQUEST_URL, "nonexistent",
             WGET_TEST_EXPECTED_ERROR_CODE, 8, 0);

  // test-stdouterr
  wget_test (WGET_TEST_OPTIONS, "-c -O /dev/full",
             WGET_TEST_REQUEST_URL, "dummy.txt",
             WGET_TEST_EXPECTED_ERROR_CODE, 3, 0);

  // test--spider
  wget_test (WGET_TEST_OPTIONS, "--spider",
             WGET_TEST_REQUEST_URL, "index.html",
             WGET_TEST_EXPECTED_ERROR_CODE, 0, 0);

  // test--spider-fail
  wget_test (WGET_TEST_OPTIONS, "--spider",
             WGET_TEST_REQUEST_URL, "nonexistent",
             WGET_TEST_EXPECTED_ERROR_CODE, 8, 0);

  // test--spider-r--no-content-disposition-trivial
  wget_test (WGET_TEST_OPTIONS, "--spider -r --no-content-disposition",
             WGET_TEST_REQUEST_URL, "", WGET_TEST_EXPECTED_ERROR_CODE, 8, 0);

  // test--no-content-disposition-trivial
  wget_test (WGET_TEST_OPTIONS, "--no-content-disposition",
             WGET_TEST_REQUEST_URL, "index.html",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "index.html", urls[0].body},
                 {
                   NULL}}, 0);

  urls[1].headers[1] =
    "Content-Disposition: attachment; filename=\"filename.html\"";

  // test--no-content-disposition
  wget_test (WGET_TEST_OPTIONS, "--no-content-disposition",
             WGET_TEST_REQUEST_URL, "index.html",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "index.html", urls[0].body},
                 {
                   NULL}}, 0);

  // test--spider-r--no-content-disposition
  wget_test (WGET_TEST_OPTIONS, "--spider -r --no-content-disposition",
             WGET_TEST_REQUEST_URL, "", WGET_TEST_EXPECTED_ERROR_CODE, 8, 0);

  urls[1].headers[1] = NULL;

  // test--HTTP-content-disposition
  wget_test (WGET_TEST_OPTIONS, "-e contentdisposition=on",
             WGET_TEST_REQUEST_URL, "dummy.html",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "filename.html", dummypage},
                 {
                   NULL}}, 0);

  // test--HTTP-content-disposition-1
  wget_test (WGET_TEST_OPTIONS, "-e contentdisposition=on",
             WGET_TEST_REQUEST_URL, "dummy.html",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXISTING_FILES, &(wget_test_file_t[])
             {
               {
                 "filename.html", "dontcare"},
                 {
                   "filename.html.1", "dontcare"},
                   {
                     NULL}}, WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "filename.html", "dontcare"},
                 {
                   "filename.html.1", "dontcare"},
                   {
                     "filename.html.2", dummypage},
                     {
                       NULL}}, 0);

  // test--HTTP-content-disposition-2
  wget_test (WGET_TEST_OPTIONS, "--no-content-disposition",
             WGET_TEST_REQUEST_URL, "dummy.html",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXISTING_FILES, &(wget_test_file_t[])
             {
               {
                 "filename.html", "dontcare"},
                 {
                   "filename.html.1", "dontcare"},
                   {
                     NULL}}, WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "filename.html", "dontcare"},
                 {
                   "filename.html.1", "dontcare"},
                   {
                     "dummy.html", dummypage},
                     {
                       NULL}}, 0);

  // test--HTTP-content-disposition-RFC6266
#define ccedilla_u8 "\xC3\xA7"
  wget_test (WGET_TEST_OPTIONS,
             "-e contentdisposition=on --local-encoding=utf-8",
             WGET_TEST_REQUEST_URL, "dummy2.html",
             WGET_TEST_EXPECTED_ERROR_CODE, 0, WGET_TEST_EXISTING_FILES,
             &(wget_test_file_t[])
             {
               {
                 "filename.html", "dontcare"},
                 {
                   "filename.html.1", "dontcare"},
                   {
                     NULL}}, WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "filename.html", "dontcare"},
                 {
                   "filename.html.1", "dontcare"},
                   {
                     "file_fran" ccedilla_u8 "ais.html", dummypage},
                     {
                       NULL}}, 0);

  urls[3].headers[1] = "Last-Modified: Sat, 09 Oct 2004 08:30:00 GMT";

  // test-N--no-content-disposition-trivial
  wget_test (WGET_TEST_OPTIONS, "-N --no-content-disposition",
             WGET_TEST_REQUEST_URL, "dummy.txt",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "dummy.txt", urls[3].body, 1097310600},
                 {
                   NULL}}, 0);

  urls[3].headers[2] =
    "Content-Disposition: attachment; filename=\"filename.txt\"";

  // test-N--no-content-disposition
  wget_test (WGET_TEST_OPTIONS, "-N --no-content-disposition",
             WGET_TEST_REQUEST_URL, "dummy.txt",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "dummy.txt", urls[3].body, 1097310600},
                 {
                   NULL}}, 0);

  // test-N-HTTP--content-disposition
  wget_test (WGET_TEST_OPTIONS, "-N --content-disposition",
             WGET_TEST_REQUEST_URL, "dummy.txt",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "filename.txt", urls[3].body, 1097310600},
                 {
                   NULL}}, 0);

  urls[3].headers[2] = NULL;

  {
    // server sends same length content with slightly different content
    char modified[strlen (urls[3].body) + 1];

    strcpy (modified, urls[3].body);
    modified[3] = 'x';

    urls[3].modified = 1097310600;

    // test-N-current
    wget_test (WGET_TEST_OPTIONS, "-N",
               WGET_TEST_REQUEST_URL, "dummy.txt",
               WGET_TEST_EXPECTED_ERROR_CODE, 0,
               WGET_TEST_EXISTING_FILES, &(wget_test_file_t[])
               {
                 {
                   "dummy.txt", modified, 1097310600},
                   {
                     NULL}}, WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
               {
                 {
                   "dummy.txt", modified, 1097310600},
                   {
                     NULL}}, 0);

    // test-N-old
    wget_test (WGET_TEST_OPTIONS, "-N",
               WGET_TEST_REQUEST_URL, "dummy.txt",
               WGET_TEST_EXPECTED_ERROR_CODE, 0,
               WGET_TEST_EXISTING_FILES, &(wget_test_file_t[])
               {
                 {
                   "dummy.txt", modified, 1097310000},  // earlier timestamp
                   {
                     NULL}}, WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
               {
                 {
                   "dummy.txt", urls[3].body, 1097310600},
                   {
                     NULL}}, 0);

    /*
    // test-N-smaller
    // This test just works with a HEAD request. But Wget uses If-Modified-Since.
    const char *old_body = urls[3].body;
    modified[strlen(modified)-2] = 0;
    urls[3].body = modified;
    wget_test(
    WGET_TEST_OPTIONS, "-N",
    WGET_TEST_REQUEST_URL, "dummy.txt",
    WGET_TEST_EXPECTED_ERROR_CODE, 0,
    WGET_TEST_EXISTING_FILES, &(wget_test_file_t []) {
    {  "dummy.txt", old_body, 1097310600 },
    {  NULL } },
    WGET_TEST_EXPECTED_FILES, &(wget_test_file_t []) {
    {  "dummy.txt", modified, 1097310600 },
    {  NULL } },
    0);
    urls[3].body = old_body; // restore body
    */
  }

  // test-N
  wget_test (WGET_TEST_OPTIONS, "-N",
             WGET_TEST_REQUEST_URL, "dummy.txt",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "dummy.txt", urls[3].body, 0},
                 {
                   NULL}}, 0);

  urls[3].headers[1] = NULL;

  /*
  // test-N-no-info
  // This test just works with a HEAD request. But Wget uses If-Modified-Since.
  wget_test(
  WGET_TEST_OPTIONS, "-N",
  WGET_TEST_REQUEST_URL, "dummy.txt",
  WGET_TEST_EXPECTED_ERROR_CODE, 0,
  WGET_TEST_EXISTING_FILES, &(wget_test_file_t []) {
  {  "dummy.txt", "anycontent", 1097310600 },
  {  NULL } },
  WGET_TEST_EXPECTED_FILES, &(wget_test_file_t []) {
  {  "dummy.txt", urls[3].body, 0},
  {  NULL } },
  0);
  */

  urls[1].headers[1] =
    "Content-Disposition: attachment; filename=\"filename.html\"";

  // test-O--no-content-disposition
  wget_test (WGET_TEST_OPTIONS, "-O out --no-content-disposition",
             WGET_TEST_REQUEST_URL, "dummy.txt",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "out", urls[3].body},
                 {
                   NULL}}, 0);

  // test-O-HTTP-content-disposition
  wget_test (WGET_TEST_OPTIONS, "-O out",
             WGET_TEST_REQUEST_URL, "dummy.txt",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "out", urls[3].body},
                 {
                   NULL}}, 0);


  urls[3].headers[1] = NULL;

  // test-O-nc
  wget_test (WGET_TEST_OPTIONS, "-nc -O out",
             WGET_TEST_REQUEST_URL, "dummy.txt",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "out", urls[3].body},
                 {
                   NULL}}, 0);

  // test-O-nonexisting
  wget_test (WGET_TEST_OPTIONS, "-O out",
             WGET_TEST_REQUEST_URL, "nonexistent",
             WGET_TEST_EXPECTED_ERROR_CODE, 8,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               // {  "out", "" }, // Wget would create an empty file here, but Wget not
               {
                 NULL}}, 0);

  // test-O
  wget_test (WGET_TEST_OPTIONS, "-O out",
             WGET_TEST_REQUEST_URL, "dummy.txt",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "out", urls[3].body},
                 {
                   NULL}}, 0);

  // test-restrict-lowercase
  urls[3].name = "/DuMmy.Txt";
  wget_test (WGET_TEST_OPTIONS, "--restrict-file-names=lowercase",
             WGET_TEST_REQUEST_URL, "DuMmy.Txt",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "dummy.txt", urls[3].body},
                 {
                   NULL}}, 0);

  // test-restrict-uppercase
  wget_test (WGET_TEST_OPTIONS, "--restrict-file-names=uppercase",
             WGET_TEST_REQUEST_URL, "DuMmy.Txt",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "DUMMY.TXT", urls[3].body},
                 {
                   NULL}}, 0);
  urls[3].name = "/dummy.txt";

  // test-c-full
  wget_test (WGET_TEST_OPTIONS, "-c",
             WGET_TEST_REQUEST_URL, "dummy.txt",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXISTING_FILES, &(wget_test_file_t[])
             {
               {
                 "dummy.txt", urls[3].body},
                 {
                   NULL}}, WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "dummy.txt", urls[3].body},
                 {
                   NULL}}, 0);

  {
    // server sends same length content with slightly different content
    char *partial = strndup (urls[3].body, strlen (urls[3].body) - 2);

    // test-c-partial
    wget_test (WGET_TEST_OPTIONS, "-c", WGET_TEST_REQUEST_URL, "dummy.txt",
               //      WGET_TEST_KEEP_TMPFILES, 1,
               WGET_TEST_EXPECTED_ERROR_CODE, 0,
               WGET_TEST_EXISTING_FILES, &(wget_test_file_t[])
               {
                 {
                   "dummy.txt", partial},
                   {
                     NULL}}, WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
               {
                 {
                   "dummy.txt", urls[3].body},
                   {
                     NULL}}, 0);

    wget_xfree (partial);
  }

  /*
   * this test needs a broken server ... I don't have one right now.
   {
   // server sends same length content with slightly different content
   char *partial = strndup(urls[3].body, strlen(urls[3].body)-2);
   const char *old_body = urls[3].body;
   urls[3].body = "";

   // test-c-shorter
   wget_test(
   WGET_TEST_OPTIONS, "-d -c",
   WGET_TEST_REQUEST_URL, "dummy.txt",
   //      WGET_TEST_KEEP_TMPFILES, 1,
   WGET_TEST_EXPECTED_ERROR_CODE, 0,
   WGET_TEST_EXISTING_FILES, &(wget_test_file_t []) {
   {  "dummy.txt", partial },
   {  NULL } },
   WGET_TEST_EXPECTED_FILES, &(wget_test_file_t []) {
   {  "dummy.txt", urls[3].body },
   {  NULL } },
   0);

   urls[3].body = old_body;
   wget_xfree(partial);
   }
  */
  // test-c
  wget_test (WGET_TEST_OPTIONS, "-c",
             WGET_TEST_REQUEST_URL, "dummy.txt",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             // no existing file
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "dummy.txt", urls[3].body},
                 {
                   NULL}}, 0);

  // test--https-only
  wget_test (WGET_TEST_OPTIONS, "--https-only -r -nH",
             WGET_TEST_REQUEST_URL, "index.html",
             WGET_TEST_EXPECTED_ERROR_CODE, 0,
             WGET_TEST_EXPECTED_FILES, &(wget_test_file_t[])
             {
               {
                 "index.html", urls[0].body},
                 {
                   NULL}}, 0);

  exit (0);
}
