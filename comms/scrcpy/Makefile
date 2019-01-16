# $FreeBSD$

PORTNAME=	scrcpy
DISTVERSIONPREFIX=	v
DISTVERSION=	1.5
CATEGORIES=	comms net
MASTER_SITES=	https://github.com/${GH_ACCOUNT}/${PORTNAME}/releases/download/${GH_TAGNAME}/:jar
DISTFILES=	${PORTNAME}-server-${DISTVERSIONPREFIX}${DISTVERSION}.jar:jar
EXTRACT_ONLY=	${DISTNAME}${EXTRACT_SUFX}

MAINTAINER=	tagattie@yandex.com
COMMENT=	Display and control your Android device

LICENSE=	APACHE20
LICENSE_FILE=	${WRKSRC}/LICENSE

LIB_DEPENDS=	libavformat.so:multimedia/ffmpeg \
		libavcodec.so:multimedia/ffmpeg \
		libavutil.so:multimedia/ffmpeg
RUN_DEPENDS=	adb:devel/android-tools-adb

USES=		meson pkgconfig

USE_SDL=	sdl2

USE_GITHUB=	yes
GH_ACCOUNT=	Genymobile
GH_TAGNAME=	${DISTVERSIONPREFIX}${DISTVERSION}-fixversion

MESON_ARGS=	-Dbuild_server=false

SERVERJARNAME=	${PORTNAME}-server.jar
PLIST_FILES=	bin/scrcpy ${DATADIR}/${SERVERJARNAME}

post-extract:
	${CP} ${_DISTDIR}/scrcpy-server-${DISTVERSIONPREFIX}${DISTVERSION}.jar ${WRKSRC}/${SERVERJARNAME}

post-install:
	${MKDIR} ${STAGEDIR}${DATADIR}
	${INSTALL_DATA} ${WRKSRC}/${SERVERJARNAME} ${STAGEDIR}${DATADIR}

.include <bsd.port.mk>
