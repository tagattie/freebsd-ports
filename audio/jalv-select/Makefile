PORTNAME=	jalv-select
DISTVERSIONPREFIX=	v
DISTVERSION=	1.3
PORTREVISION=	6
CATEGORIES=	audio

MAINTAINER=	yuri@FreeBSD.org
COMMENT=	Little app to select lv2 plugins to run with jalv
WWW=		https://github.com/brummer10/jalv_select

LICENSE=	UNLICENSE
LICENSE_FILE=	${WRKSRC}/LICENSE

BUILD_DEPENDS=	lv2>0:audio/lv2
LIB_DEPENDS=	libfontconfig.so:x11-fonts/fontconfig \
		libfreetype.so:print/freetype2 \
		liblilv-0.so:audio/lilv \
		libserd-0.so:devel/serd \
		libsord-0.so:devel/sord \
		libsratom-0.so:audio/sratom
RUN_DEPENDS=	gawk:lang/gawk \
		jalv:audio/jalv

USES=		compiler:c++11-lang gettext-runtime gmake gnome pkgconfig xorg
USE_GITHUB=	yes
GH_ACCOUNT=	brummer10
GH_PROJECT=	jalv_select
USE_GNOME=	atk atkmm cairo gdkpixbuf glib20 glibmm gtk20 gtkmm24 pango pangomm
USE_XORG=	x11

MAKE_ARGS=	MAN_DIR=${PREFIX}/share/man/man1

post-install:
	@${STRIP_CMD} ${STAGEDIR}${PREFIX}/bin/jalv.select

.include <bsd.port.mk>
