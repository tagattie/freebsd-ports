# $FreeBSD$

PORTNAME=	vgmplay
PORTVERSION=	0.40.9
CATEGORIES=	audio
MASTER_SITES=	https://github.com/vgmrips/${PORTNAME}/releases/download/${PORTVERSION}/
DISTNAME=	${PORTNAME:S/vgmp/VGMP/}_${PORTVERSION:S/.//:S/./-/}_src

MAINTAINER=	tagattie@yandex.com
COMMENT=	Command-line player for VGM (video game music) files

LICENSE=	GPLv2
LICENSE_FILE=	${WRKSRC}/licenses/GPL.txt

USES=		7z dos2unix gmake

NO_WRKSUBDIR=	yes

OPTIONS_SINGLE=	AUDIO
OPTIONS_SINGLE_AUDIO=	OSS LIBAO
OPTIONS_DEFAULT=	OSS

OSS_DESC=	Use OSS (/dev/dsp)
LIBAO_DESC=	Use libao

OSS_MAKE_ENV+=	USE_LIBAO=0
LIBAO_MAKE_ENV+=	USE_LIBAO=1
LIBAO_LIB_DEPENDS=	libao.so:audio/libao
LIBAO_CFLAGS+=	-I${LOCALBASE}/include
LIBAO_LDFLAGS+=	-L${LOCALBASE}/lib

MAKE_ENV+=	PREFIX=${LOCALBASE}

post-patch:
	${REINPLACE_CMD} -e 's/@//' ${WRKSRC}/Makefile
	${REINPLACE_CMD} -e 's|/usr/local|${LOCALBASE}|; s|/usr/share/vgmplay|${EXAMPLESDIR}|; s|VGMPlay|vgmplay|g;' ${WRKSRC}/vgmplay.1

.include <bsd.port.mk>
