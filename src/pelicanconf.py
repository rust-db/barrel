#!/usr/bin/env python
# -*- coding: utf-8 -*- #
from __future__ import unicode_literals

AUTHOR = 'Sql People'
SITENAME = 'barrel.rs'
SITEURL = ''

THEME = 'aboutwilson'

EXTRA_PATH_METADATA = {
  'favicon.ico': {'path': 'favicon.ico'}
}

TEMPLATE_DEBUG = True
DEBUG = True
READ_TIME = 180

PATH = 'content'
STATIC_PATHS = ['images', 'downloads', 'kookie.txt']
SITE_LOGO = 'favicon.ico'

SUMMARY_MAX_LENGTH = 140

#############################################
#############################################

DEFAULT_CATEGORY = 'Blog'
DEFAULT_DATE = 'fs'

DISPLAY_CATEGORIES_ON_MENU = False
DISPLAY_PAGES_ON_MENU = False

MENUITEMS = (
  ('About', '/'),
  ('Blog', '/blog/'),
)

ARTICLE_URL = '{category}/{slug}'
ARTICLE_SAVE_AS = '{category}/{slug}/index.html'

PAGE_URL = '{slug}'
PAGE_SAVE_AS = '{slug}/index.html'

CATEGORY_URL = '{slug}'
CATEGORY_SAVE_AS = '{slug}/index.html'

TAG_URL = '{slug}'
TAG_SAVE_AS = '{slug}/index.html'

#############################################
#############################################

TIMEZONE = 'UTC'
DEFAULT_LANG = 'en'
LOCALE = 'C'
DEFAULT_DATE_FORMAT = '%Y-%m-%d'

# Feed generation is usually not desired when developing
FEED_ALL_ATOM = None
CATEGORY_FEED_ATOM = None
TRANSLATION_FEED_ATOM = None
AUTHOR_FEED_ATOM = None

FEED_RSS = 'rss.xml'
CATEGORY_FEED_RSS = '%s/rss.xml'

JINJA_ENVIRONMENT = {
  'extensions': ['webassets.ext.jinja2.AssetsExtension', 'jinja2.ext.with_']
}

DEFAULT_PAGINATION = 20

