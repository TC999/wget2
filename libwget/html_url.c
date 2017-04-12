/*
 * Copyright(c) 2013 Tim Ruehsen
 * Copyright(c) 2015-2016 Free Software Foundation, Inc.
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
 * along with libwget.  If not, see <https://www.gnu.org/licenses/>.
 *
 *
 * Extracting URLs from HTML
 *
 * Changelog
 * 26.09.2013  Tim Ruehsen  created
 *
 */

#if HAVE_CONFIG_H
# include <config.h>
#endif

#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include <c-ctype.h>

#include <wget.h>
#include "private.h"

typedef struct {
	WGET_HTML_PARSED_RESULT
		result;
	wget_vector_t *
		additional_tags;
	wget_vector_t *
		ignore_tags;
	char
		found_robots,
		found_content_type,
		link_inline;
	const char
		* css_attr,
		* css_dir;
} _html_context_t;

// see https://stackoverflow.com/questions/2725156/complete-list-of-html-tag-attributes-which-have-a-url-value
static const char maybe[256] = {
	['a'] = 1,
	['b'] = 1,
	['c'] = 1,
	['d'] = 1,
	['f'] = 1,
	['h'] = 1,
	['i'] = 1,
	['l'] = 1,
	['m'] = 1,
	['p'] = 1,
	['s'] = 1,
	['u'] = 1,
};
static const char attrs[][12] = {
	"action", "archive",
	"background",
	"code", "codebase", "cite", "classid",
	"data",
	"formaction",
	"href",
	"icon",
	"lowsrc", "longdesc",
	"manifest",
	"profile", "poster",
	"src", "srcset",
	"usemap"
};

static void _css_parse_encoding(void *context, const char *encoding, size_t len)
{

}

static void _css_parse_uri(void *context, const char *url, size_t len, size_t pos G_GNUC_WGET_UNUSED)
{
	_html_context_t *ctx = context;
	
	WGET_HTML_PARSED_RESULT *res = &ctx->result;
	
	if (!res->uris)
		res->uris = wget_vector_create(32, -2, NULL);

	WGET_HTML_PARSED_URL parsed_url;
	parsed_url.link_inline = 1;
	strlcpy(parsed_url.attr, ctx->css_attr, sizeof(parsed_url.attr));
	strlcpy(parsed_url.dir, ctx->css_dir, sizeof(parsed_url.dir));
	parsed_url.url.p = wget_strmemdup(url, len);
	parsed_url.url.len = len;
	parsed_url.must_free_url = 1;
	
	wget_vector_add(res->uris, &parsed_url, sizeof(parsed_url));
}

// Callback function, called from HTML parser for each URI found.
static void _html_get_url(void *context, int flags, const char *tag, const char *attr, const char *val, size_t len, size_t pos G_GNUC_WGET_UNUSED)
{
	_html_context_t *ctx = context;

	// Read the encoding from META tag, e.g. from
	//   <meta http-equiv="Content-Type" content="text/html; charset=utf-8">.
	// It overrides the encoding from the HTTP response resp. from the CLI.
	//
	// Also ,we are interested in ROBOTS e.g.
	//   <META name="ROBOTS" content="NOINDEX, NOFOLLOW">
	if ((flags & XML_FLG_BEGIN)) {
		if ((*tag|0x20) == 'm' && !wget_strcasecmp_ascii(tag, "meta"))
			ctx->found_robots = ctx->found_content_type = 0;
		
		if ((*tag|0x20) == 'l' && !wget_strcasecmp_ascii(tag, "link"))
			ctx->link_inline = 1;
		else
			ctx->link_inline = 0;
	}

	if ((flags & XML_FLG_ATTRIBUTE) && val) {
		WGET_HTML_PARSED_RESULT *res = &ctx->result;

//		info_printf("%02X %s %s '%.*s' %zu %zu\n", (unsigned) flags, tag, attr, (int) len, val, len, pos);

		if ((*tag|0x20) == 'm' && !wget_strcasecmp_ascii(tag, "meta")) {
			if (!ctx->found_robots) {
				if (!wget_strcasecmp_ascii(attr, "name") && !wget_strncasecmp_ascii(val, "robots", len)) {
					ctx->found_robots = 1;
					return;
				}
			} else if (ctx->found_robots && !wget_strcasecmp_ascii(attr, "content")) {
				char *p;
				char valbuf[len + 1], *value = valbuf;

				memcpy(value, val, len);
				value[len] = 0;

				while (*value) {
					while (c_isspace(*value)) value++;
					if (*value == ',') { value++; continue; }
					for (p = value; *p && !c_isspace(*p) && *p != ','; p++);
					if (p == value) break;

					// debug_printf("ROBOTS='%.*s'\n", (int)(p - value), value);
					if (!wget_strncasecmp_ascii(value, "all", p - value) || !wget_strncasecmp_ascii(value, "follow", p - value))
						res->follow = 1;
					else if (!wget_strncasecmp_ascii(value, "nofollow", p - value) || !wget_strncasecmp_ascii(value, "none", p - value))
						res->follow = 0;

					value = *p  ? p + 1 : p;
				}
				return;
			}

			if (ctx->found_content_type && !res->encoding) {
				if (!wget_strcasecmp_ascii(attr, "content")) {
					char valbuf[len + 1], *value = valbuf;

					memcpy(value, val, len);
					value[len] = 0;
					wget_http_parse_content_type(value, NULL, &res->encoding);
				}
			}
			else if (!ctx->found_content_type && !res->encoding) {
				if (!wget_strcasecmp_ascii(attr, "http-equiv") && !wget_strncasecmp_ascii(val, "Content-Type", len)) {
					ctx->found_content_type = 1;
				}
				else if (!wget_strcasecmp_ascii(attr, "charset")) {
					res->encoding = wget_strmemdup(val, len);
				}
			}

			return;
		}

		if (ctx->ignore_tags) {
			if (wget_vector_find(ctx->ignore_tags, &(wget_html_tag_t){ .name = tag, .attribute = NULL } ) != -1
				|| wget_vector_find(ctx->ignore_tags, &(wget_html_tag_t){ .name = tag, .attribute = attr } ) != -1)
				return;
		}
		
		if ((*attr|0x20) == 's' && !wget_strcasecmp_ascii(attr, "style") && len) {
			ctx->css_dir = tag;
			ctx->css_attr = "style";
			wget_css_parse_buffer(val, len, _css_parse_uri, _css_parse_encoding, context);
			return;
		}

		// shortcut to avoid unneeded calls to bsearch()
		int found = 0;

		// search the static list for a tag/attr match
		if (maybe[(unsigned char)*attr|0x20] && attr[1] && attr[2])
			found = bsearch(attr, attrs, countof(attrs), sizeof(attrs[0]), (int(*)(const void *, const void *))wget_strcasecmp_ascii) != NULL;

		// search the dynamic list for a tag/attr match
		if (!found && ctx->additional_tags) {
			if (wget_vector_find(ctx->additional_tags, &(wget_html_tag_t){ .name = tag, .attribute = NULL } ) != -1
				|| wget_vector_find(ctx->additional_tags, &(wget_html_tag_t){ .name = tag, .attribute = attr } ) != -1)
				found = 1;
		}

		if (found) {
			for (;len && c_isspace(*val); val++, len--); // skip leading spaces
			for (;len && c_isspace(val[len - 1]); len--);  // skip trailing spaces

			if ((*tag|0x20) == 'b' && !wget_strcasecmp_ascii(tag, "base")) {
				// found a <BASE href="...">
				res->base.p = val;
				res->base.len = len;
				return;
			}

			if (!res->uris)
				res->uris = wget_vector_create(32, -2, NULL);

			WGET_HTML_PARSED_URL url;
			url.must_free_url = 0;

			if (!wget_strcasecmp_ascii(attr, "srcset")) {
				// value is a list of URLs, see https://html.spec.whatwg.org/multipage/embedded-content.html#attr-img-srcset
				while (len) {
					const char *p;

					for (;len && c_isspace(*val); val++, len--); // skip leading spaces
					for (p = val;len && !c_isspace(*val) && *val != ','; val++, len--); // find end of URL
					if (p != val) {
						strlcpy(url.attr, attr, sizeof(url.attr));
						strlcpy(url.dir, tag, sizeof(url.dir));
						url.url.p = p;
						url.url.len = val - p;
						wget_vector_add(res->uris, &url, sizeof(url));
					}
					for (;len && *val != ','; val++, len--); // skip optional width/density descriptor
					if (len && *val == ',') { val++; len--; }
				}

			} else {
				// value is a single URL
				url.link_inline = ctx->link_inline;
				strlcpy(url.attr, attr, sizeof(url.attr));
				strlcpy(url.dir, tag, sizeof(url.dir));
				url.url.p = val;
				url.url.len = len;
				wget_vector_add(res->uris, &url, sizeof(url));
			}
		}
	}
	
	if (flags & XML_FLG_CONTENT && val && len && !wget_strcasecmp_ascii(tag, "style")) {
		ctx->css_dir = "style";
		ctx->css_attr = "";
		wget_css_parse_buffer(val, len, _css_parse_uri, _css_parse_encoding, context);
	}
}

/*
static void _urls_to_absolute(WGET_VECTOR *urls, WGET_IRI *base)
{
	if (base && urls) {
		wget_buffer_t buf;
		wget_buffer_init(&buf, NULL, 1024);

		for (int it = 0; it < wget_vector_size(urls); it++) {
			WGET_PARSED_URL *url = wget_vector_get(urls, it);

			if (wget_iri_relative_to_abs(base, url->url, url->len, &buf))
				url->abs_url = wget_strmemdup(buf.data, buf.length);
			else
				error_printf("Cannot resolve relative URI '%s'\n", url->url);
		}

		wget_buffer_deinit(&buf);
	}
}
*/

void wget_html_free_urls_inline (WGET_HTML_PARSED_RESULT **res)
{
	if (res && *res) {
		xfree((*res)->encoding);
		
		for (int it = 0; it < wget_vector_size((*res)->uris); it++) {
			WGET_HTML_PARSED_URL *html_url = wget_vector_get((*res)->uris, it);
			
			if(html_url->must_free_url) {
				xfree(html_url->url.p);
			}
		}
		wget_vector_free(&(*res)->uris);
		
		xfree(*res);
	}
}

WGET_HTML_PARSED_RESULT *wget_html_get_urls_inline(const char *html, wget_vector_t *additional_tags, wget_vector_t *ignore_tags)
{
	_html_context_t context = {
		.result.follow = 1,
		.additional_tags = additional_tags,
		.ignore_tags = ignore_tags,
	};

//	context.result.uris = wget_vector_create(32, -2, NULL);
	wget_html_parse_buffer(html, _html_get_url, &context, HTML_HINT_REMOVE_EMPTY_CONTENT);

	return wget_memdup(&context.result, sizeof(context.result));
}
