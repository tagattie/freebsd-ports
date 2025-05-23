PORTNAME=	hedgewars
PORTVERSION=	1.0.2
PORTREVISION=	5
CATEGORIES=	games
MASTER_SITES=	https://www.hedgewars.org/download/releases/
DISTNAME=	${PORTNAME}-src-${DISTVERSION}

MAINTAINER=	amdmi3@FreeBSD.org
COMMENT=	Free Worms-like turn based strategy game
WWW=		https://www.hedgewars.org

LICENSE=	GPLv2 GFDL
LICENSE_COMB=	multi

LIB_DEPENDS=	libphysfs.so:devel/physfs \
		libpng.so:graphics/png

ONLY_FOR_ARCHS=	amd64 i386

USES=		cmake:noninja desktop-file-utils fpc gl lua:51 pkgconfig qt:5 \
		sdl tar:bzip2
USE_SDL=	sdl2 mixer2 image2 ttf2 net2
USE_FPC=	opengl libpng rtl-objpas rtl-extra
USE_QT=		core gui widgets network \
		qmake:build buildtools:build \
		linguisttools:build
USE_GL=		gl glu
USE_LDCONFIG=	yes
CMAKE_ON=	NOSERVER

PLIST_FILES=	bin/${PORTNAME} bin/hwengine \
		share/appdata/hedgewars.appdata.xml \
		share/applications/${PORTNAME}.desktop \
		share/pixmaps/${PORTNAME}.png \
		share/pixmaps/${PORTNAME}.xpm \
		lib/libphyslayer.so \
		lib/libphyslayer.so.1.0
PORTDATA=	*

OPTIONS_DEFINE=	VIDEOREC
OPTIONS_DEFAULT=VIDEOREC
OPTIONS_EXCLUDE_i386=	VIDEOREC  # some pascal issues preventing to use it

VIDEOREC_DESC=	Enable video recording (requires ffmpeg)

VIDEOREC_LIB_DEPENDS=	libavcodec.so.58:multimedia/ffmpeg4
VIDEOREC_PLIST_FILES=	lib/libavwrapper.so \
			lib/libavwrapper.so.1.0
VIDEOREC_CMAKE_OFF=	-DNOVIDEOREC=1

.include <bsd.port.pre.mk>

.if ${ARCH} == i386
BUILD_DEPENDS+=	ghc:lang/ghc
.endif

post-extract:
	# Remove included CMakeFile, which is incompatible with >= cmake-3.24
	${RM} ${WRKSRC}/cmake_modules/CMakeSystemSpecificInformation.cmake

post-patch:
	@${REINPLACE_CMD} -e 's|/usr/local|${LOCALBASE}|' ${WRKSRC}/cmake_modules/*.cmake
	@${REINPLACE_CMD} -e "s|'liblua'|'liblua-${LUA_VER}'|" ${WRKSRC}/hedgewars/LuaPas.pas
	@${REINPLACE_CMD} -e "/linklib/ s|lua|&-${LUA_VER}|" ${WRKSRC}/hedgewars/LuaPas.pas
	@${REINPLACE_CMD} -e 's|%%LOCALBASE%%|${LOCALBASE}|g' \
		${PATCH_WRKSRC}/cmake_modules/FindLIBAV.cmake

post-install:
	@${STRIP_CMD} ${STAGEDIR}${PREFIX}/bin/*
	${INSTALL_DATA} ${WRKSRC}/share/hedgewars/Data/misc/hedgewars.desktop \
		${STAGEDIR}${PREFIX}/share/applications/
	${INSTALL_DATA} ${WRKSRC}/misc/hedgewars.png \
		${STAGEDIR}${PREFIX}/share/pixmaps/

.include <bsd.port.post.mk>
