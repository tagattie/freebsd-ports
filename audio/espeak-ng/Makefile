PORTNAME=	espeak-ng
PORTVERSION=	1.52.0
PORTREVISION=	1
CATEGORIES=	audio
#MASTER_SITES=	https://github.com/espeak-ng/${PORTNAME}/releases/download/${PORTVERSION}/

MAINTAINER=	se@FreeBSD.org
COMMENT=	Software speech synthesizer
WWW=		https://github.com/espeak-ng/espeak-ng/

LICENSE=	GPLv3

BUILD_DEPENDS=	ronn:textproc/rubygem-ronn
LIB_DEPENDS=	libpcaudio.so:audio/pcaudiolib

USES=		autoreconf gmake libtool localbase
USE_GITHUB=	yes
USE_LDCONFIG=	yes

GNU_CONFIGURE=	yes
GNU_CONFIGURE_MANPREFIX=${PREFIX}/share
CONFIGURE_ARGS=	--with-pcaudiolib

CONFLICTS_INSTALL=	espeak speak

OPTIONS_DEFINE=	DOCS

pre-build:
	@${REINPLACE_CMD} -e 's|%%DATADIR%%|${DATADIR}|' \
		${WRKSRC}/Makefile.am ${WRKSRC}/src/libespeak-ng/speech.h

post-install:
	${MKDIR} ${STAGEDIR}${DOCSDIR}
	cd ${WRKSRC}/docs && ${COPYTREE_SHARE} . ${STAGEDIR}${DOCSDIR}
	${RLN} ${STAGEDIR}${PREFIX}/share/man/man1/espeak-ng.1 \
			${STAGEDIR}${PREFIX}/share/man/man1/espeak.1
	${LN} -sf libespeak-ng.so ${STAGEDIR}${PREFIX}/lib/libespeak.so
	${STRIP_CMD} ${STAGEDIR}${PREFIX}/bin/${PORTNAME}
	${STRIP_CMD} ${STAGEDIR}${PREFIX}/lib/libespeak-ng.so*

.include <bsd.port.mk>
