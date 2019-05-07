#!/usr/bin/env python3
# -*- coding: utf8 -*-
# :Copyright: © 2015 Günter Milde.
# :License: Released under the terms of the `2-Clause BSD license`_, in short:
#
#    Copying and distribution of this file, with or without modification,
#    are permitted in any medium without royalty provided the copyright
#    notice and this notice are preserved.
#    This file is offered as-is, without any warranty.
#
# .. _2-Clause BSD license: http://www.spdx.org/licenses/BSD-2-Clause
#
# Revision: $Revision: 7847 $
# Date: $Date: 2015-03-17 18:30:47 +0100 (Di, 17 Mär 2015) $

"""
A minimal front end to the Docutils Publisher, producing HTML 5 documents.

The output also conforms to XHTML 1.0 transitional
(except for the doctype declaration).
"""

try:
    import locale # module missing in Jython
    locale.setlocale(locale.LC_ALL, '')
except locale.Error:
    pass

from docutils import nodes
from docutils.core import publish_cmdline, default_description
from docutils.parsers.rst import roles

# https://github.com/github/markup/blob/master/lib/github/commands/rest2html#L233
def kbd(name, rawtext, text, lineno, inliner, options={}, content=[]):
    return [nodes.raw('', '<kbd>%s</kbd>' % text, format='html')], []

roles.register_canonical_role('kbd', kbd)

description = (u'Generates HTML 5 documents from standalone '
               u'reStructuredText sources '
               + default_description)

publish_cmdline(writer_name='html5', description=description)
