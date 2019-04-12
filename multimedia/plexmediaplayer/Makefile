# $FreeBSD$

PORTNAME=	plexmediaplayer
DISTVERSIONPREFIX=	v
DISTVERSION=	2.31.0
DISTVERSIONSUFFIX=	.967-a95b6d76
CATEGORIES=	multimedia
MASTER_SITES=	https://artifacts.plex.tv/web-client-pmp/${WEB_CLIENT_BUILD_ID}/
DISTFILES=	buildid.cmake \
		web-client-desktop-${DESKTOP_VER}.tar.xz \
		web-client-tv-${TV_VER}.tar.xz
EXTRACT_ONLY=	${DISTNAME}${EXTRACT_SUFX} \
		web-client-desktop-${DESKTOP_VER}.tar.xz \
		web-client-tv-${TV_VER}.tar.xz

MAINTAINER=	tagattie@yandex.com
COMMENT=	Next generation Plex desktop/embedded client

LICENSE=	GPLv2
LICENSE_FILE=	${WRKSRC}/LICENSE

LIB_DEPENDS=	libmpv.so:multimedia/mpv
RUN_DEPENDS=	xdg-open:devel/xdg-utils

USES=		cmake desktop-file-utils gnome ninja qt:5
USE_QT=		core network quickcontrols webchannel webengine x11extras xml

USE_GITHUB=	yes
GH_ACCOUNT=	plexinc
GH_PROJECT=	plex-media-player

# See ${WRKSRC}/CMakeModules/WebClient.cmake for WEB_CLIENT_BUILD_ID
WEB_CLIENT_BUILD_ID=	113-862bda5ecd072f
# Values for these two variables are found in downloaded buildid.cmake
DESKTOP_VER=	3.83.2-ecd072f
TV_VER=		3.94.0-862bda5

CMAKE_ARGS=	-DQTROOT=${LOCALBASE}

OPTIONS_DEFINE=	DBUS HELPER
OPTIONS_GROUP=	INPUT
OPTIONS_GROUP_INPUT=	CEC JOYSTICK LIRC
OPTIONS_DEFAULT=	DBUS HELPER CEC JOYSTICK LIRC

DBUS_DESC=	Enable D-Bus screensaver management
DBUS_CMAKE_BOOL_OFF=	LINUX_X11POWER
HELPER_DESC=	Enable PMP Helper
HELPER_CMAKE_BOOL=	ENABLE_HELPER
CEC_DESC=	Enable HDMI/CEC support
CEC_CMAKE_BOOL=	ENABLE_CEC
CEC_LIB_DEPENDS=	libcec.so:multimedia/libcec
JOYSTICK_DESC=	Enable joystick support
JOYSTICK_CMAKE_BOOL=	ENABLE_SDL2
JOYSTICK_USES=	sdl
JOYSTICK_USE=	sdl=sdl2
LIRC_DESC=	Enable LIRC support
LIRC_CMAKE_BOOL=	ENABLE_LIRC
LIRC_RUN_DEPENDS=	lircd:comms/lirc

INSTALLS_ICONS=	yes

pre-fetch:
	${RM} ${_DISTDIR}/buildid.cmake

post-extract:
	${MKDIR} ${BUILD_WRKSRC}/dependencies
	${CP} ${_DISTDIR}/buildid.cmake ${BUILD_WRKSRC}/dependencies/buildid-${WEB_CLIENT_BUILD_ID}.cmake
	${MKDIR} ${BUILD_WRKSRC}/dependencies/universal-web-client-desktop/${WEB_CLIENT_BUILD_ID}
	${MV} ${WRKDIR}/web-client-desktop-${DESKTOP_VER} ${BUILD_WRKSRC}/dependencies/universal-web-client-desktop/${WEB_CLIENT_BUILD_ID}
	${TOUCH} ${BUILD_WRKSRC}/dependencies/universal-web-client-desktop/${WEB_CLIENT_BUILD_ID}/web-client-desktop-${DESKTOP_VER}/_FINISHED
	${MKDIR} ${BUILD_WRKSRC}/dependencies/universal-web-client-tv/${WEB_CLIENT_BUILD_ID}
	${MV} ${WRKDIR}/web-client-tv-${TV_VER} ${BUILD_WRKSRC}/dependencies/universal-web-client-tv/${WEB_CLIENT_BUILD_ID}
	${TOUCH} ${BUILD_WRKSRC}/dependencies/universal-web-client-tv/${WEB_CLIENT_BUILD_ID}/web-client-tv-${TV_VER}/_FINISHED

.include <bsd.port.mk>
