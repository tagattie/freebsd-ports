# $FreeBSD$

PORTNAME=	feedreader
DISTVERSIONPREFIX=	v
DISTVERSION=	2.8.2
CATEGORIES=	net

MAINTAINER=	tagattie@yandex.com
COMMENT=	Desktop application to complement existing web-based RSS accounts

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/LICENSE

BUILD_DEPENDS=	appstream-util:devel/appstream-glib \
		valac:lang/vala
LIB_DEPENDS=	libgumbo.so:devel/gumbo \
		libjson-glib-1.0.so:devel/json-glib \
		libgee-0.8.so:devel/libgee \
		libnotify.so:devel/libnotify \
		libpeas-1.0.so:devel/libpeas \
		librest-0.7.so:devel/librest \
		libsoup-2.4.so:devel/libsoup \
		libcurl.so:ftp/curl \
		libgoa-1.0.so:net/gnome-online-accounts \
		libsecret-1.so:security/libsecret \
		libwebkit2gtk-4.0.so:www/webkit2-gtk3

USES=		desktop-file-utils gettext-runtime gnome localbase:ldflags \
		meson python:3.5+,build shebangfix sqlite

USE_GITHUB=	yes
GH_ACCOUNT=	jangernert
GH_PROJECT=	FeedReader

USE_GNOME=	gdkpixbuf2 gtk30 libxml2
USE_GSTREAMER1=	yes

SHEBANG_FILES=	meson_post_install.py

GLIB_SCHEMAS=	org.gnome.feedreader.bazqux.gschema.xml \
		org.gnome.feedreader.decsync.gschema.xml \
		org.gnome.feedreader.feedbin.gschema.xml \
		org.gnome.feedreader.feedhq.gschema.xml \
		org.gnome.feedreader.feedly.gschema.xml \
		org.gnome.feedreader.fresh.gschema.xml \
		org.gnome.feedreader.gschema.xml \
		org.gnome.feedreader.inoreader.gschema.xml \
		org.gnome.feedreader.keybindings.gschema.xml \
		org.gnome.feedreader.oldreader.gschema.xml \
		org.gnome.feedreader.owncloud.gschema.xml \
		org.gnome.feedreader.saved-state.gschema.xml \
		org.gnome.feedreader.share.gschema.xml \
		org.gnome.feedreader.share.instapaper.gschema.xml \
		org.gnome.feedreader.share.pocket.gschema.xml \
		org.gnome.feedreader.share.telegram.gschema.xml \
		org.gnome.feedreader.share.twitter.gschema.xml \
		org.gnome.feedreader.share.wallabag.gschema.xml \
		org.gnome.feedreader.ttrss.gschema.xml \
		org.gnome.feedreader.tweaks.gschema.xml
INSTALLS_ICONS=	yes
USE_LDCONFIG=	yes

SUB_FILES=	pkg-message

post-install:
	cd ${WRKSRC}/data/tt-rss-feedreader-plugin && \
		${COPYTREE_SHARE} . ${STAGEDIR}${PREFIX}/share/feedreader/tt-rss-feedreader-plugin

.include <bsd.port.mk>
