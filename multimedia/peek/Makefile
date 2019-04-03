# $FreeBSD$

PORTNAME=	peek
PORTVERSION=	1.3.1
CATEGORIES=	multimedia

MAINTAINER=	tagattie@yandex.com
COMMENT=	Simple animated GIF screen recorder with an easy to use interface

LICENSE=	GPLv3+
LICENSE_FILE=	${WRKSRC}/LICENSE

BUILD_DEPENDS=	valac:lang/vala
LIB_DEPENDS=	libkeybinder-3.0.so:x11/keybinder-gtk3
RUN_DEPENDS=	gsettings-desktop-schemas>0:devel/gsettings-desktop-schemas \
		ffmpeg:multimedia/ffmpeg

USES=		cmake gettext-tools gnome ninja python:3.5+,build

USE_GITHUB=	yes
GH_ACCOUNT=	phw

USE_GNOME=	glib20 gtk30 libxml2

GLIB_SCHEMAS=	com.uploadedlobster.peek.gschema.xml
INSTALLS_ICONS=	yes

CMAKE_OFF=	GSETTINGS_COMPILE

OPTIONS_DEFINE=	GIFSKI GNOMESHELL MANPAGES
OPTIONS_DEFAULT=	GNOMESHELL MANPAGES
OPTIONS_SUB=	yes

GIFSKI_DESC=	Use gifski for improved GIF quality
GIFSKI_RUN_DEPENDS=	gifski:graphics/gifski

GNOMESHELL_DESC=	Use Gnome shell screen recorder (when available)
GNOMESHELL_CMAKE_OFF=	-DDISABLE_GNOME_SHELL:BOOL=true
GNOMESHELL_USE=		GSTREAMER1=good,ugly

MANPAGES_BUILD_DEPENDS=	txt2man:textproc/txt2man

.include <bsd.port.mk>
