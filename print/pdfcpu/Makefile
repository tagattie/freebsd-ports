PORTNAME=	pdfcpu
DISTVERSIONPREFIX=	v
DISTVERSION=	0.9.1
PORTREVISION=	2
CATEGORIES=	print

MAINTAINER=	diizzy@FreeBSD.org
COMMENT=	PDF processor written in Go
WWW=		https://github.com/pdfcpu/pdfcpu

LICENSE=	APACHE20
LICENSE_FILE=	${WRKSRC}/LICENSE.txt

USES=		go:modules

GO_MODULE=	github.com/${PORTNAME}/${PORTNAME}

PLIST_FILES=	bin/pdfcpu

GO_TARGET=	./cmd/${PORTNAME}

.include <bsd.port.mk>
