# $FreeBSD$

PORTNAME=	vgmplay
PORTVERSION=	0.40.9
CATEGORIES=	audio
MASTER_SITES=	https://github.com/vgmrips/vgmplay/releases/download/${PORTVERSION}/
DISTNAME=	${PORTNAME:S/vgmp/VGMP/}_${PORTVERSION:S/.//:S/./-/}_src

MAINTAINER=	tagattie@yandex.com
COMMENT=	Command-line player for VGM (video game music) files

LICENSE=	GPLv2

USES=		7z dos2unix gmake

NO_WRKSUBDIR=	yes
EXTRACT_WRKDIR=	${WRKSRC}

OPTIONS_RADIO=	AUDIO
OPTIONS_RADIO_AUDIO=	OSS LIBAO
OPTIONS_DEFAULT=	OSS
OSS_DESC=	Use built-in audio subsystem
LIBAO_DESC=	Use libao

OSS_MAKE_ENV+=	USE_LIBAO=0

LIBAO_MAKE_ENV+=	USE_LIBAO=1
LIBAO_RUN_DEPENDS=	libao.so:audio/libao

.include <bsd.port.mk>
