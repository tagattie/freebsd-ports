PORTNAME=	minecraft-client
PORTVERSION=	1.7.9
PORTREVISION=	7
CATEGORIES=	games java
MASTER_SITES=	https://homura.magzu.net/games/ \
		LOCAL/tagattie/${PORTNAME}/${PORTVERSION}
DISTFILES=	Minecraft.jar \
		Grass.png
DIST_SUBDIR=	${PORTNAME}/${PORTVERSION}
EXTRACT_ONLY=

DEPRECATED=	dead upstream, no longer functional
EXPIRATION_DATE=	2025-03-31

MAINTAINER=	ports@FreeBSD.org
COMMENT=	Client for the block building game
WWW=		https://minecraft.net/

RUN_DEPENDS=	${JAVALIBDIR}/lwjgl/lwjgl.jar:games/lwjgl \
		xrandr:x11/xrandr

USES=		java openal
JAVA_OS=	native
JAVA_VENDOR=	openjdk

DESKTOP_ENTRIES="Minecraft" \
		"Block building game" \
		"${PREFIX}/share/pixmaps/minecraft-client.png" \
		"${PORTNAME}" \
		"Game;Java;" \
		false

NO_ARCH=	yes
NO_BUILD=	yes
NO_WRKSUBDIR=	yes

do-install:
	@${MKDIR} ${STAGEDIR}${DATADIR}
	${INSTALL_DATA} ${DISTDIR}/${DIST_SUBDIR}/Minecraft.jar \
		${STAGEDIR}${DATADIR}/Minecraft.jar
	${INSTALL_DATA} ${DISTDIR}/${DIST_SUBDIR}/Grass.png \
		${STAGEDIR}${PREFIX}/share/pixmaps/minecraft-client.png
	${SED}  -e 's|=JAVA_HOME|=${JAVA_HOME}|' \
		-e 's|PREFIX|${PREFIX}|' \
		-e 's|JAVA_CMD|${JAVA}|' ${FILESDIR}/minecraft-client \
		> ${WRKDIR}/minecraft-client
	${SED} -e 's|%%PREFIX%%|${PREFIX}|' ${FILESDIR}/Notifier.java \
		> ${WRKDIR}/Notifier.java
	${INSTALL_SCRIPT} ${WRKDIR}/minecraft-client ${STAGEDIR}${PREFIX}/bin
	${INSTALL_SCRIPT} ${FILESDIR}/minecraft-runtime ${STAGEDIR}${DATADIR}
	${INSTALL_DATA} ${WRKDIR}/Notifier.java ${STAGEDIR}${DATADIR}

.include <bsd.port.mk>
