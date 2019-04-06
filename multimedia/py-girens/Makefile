# $FreeBSD$

PORTNAME=	girens
PORTVERSION=	1.0.4
CATEGORIES=	multimedia python
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	tagattie@yandex.com
COMMENT=	GTK client for movies, TV shows and music from your Plex library

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/COPYING

LIB_DEPENDS=	libhandy-0.0.so:x11-toolkits/libhandy
RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}PlexAPI>0:multimedia/py-PlexAPI@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}python-mpv>0:multimedia/py-python-mpv@${PY_FLAVOR}

USES=		desktop-file-utils gnome meson python:3.5+ shebangfix

USE_GITLAB=	yes
GL_SITE=	https://gitlab.gnome.org/
GL_ACCOUNT=	tijder
GL_COMMIT=	4079a196b7dbf703ee6cc86fb67c2589812673db

USE_GNOME=	gtk30 pygobject3

SHEBANG_FILES=	${WRKSRC}/build-aux/meson/postinstall.py

GLIB_SCHEMAS=	nl.g4d.Girens.gschema.xml
INSTALLS_ICONS=	yes

.include <bsd.port.mk>
