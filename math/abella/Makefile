PORTNAME=	abella
DISTVERSION=	2.0.8
PORTREVISION=	2
CATEGORIES=	math
MASTER_SITES=	http://abella-prover.org/distributions/

MAINTAINER=	yuri@FreeBSD.org
COMMENT=	Interactive theorem prover
WWW=		https://abella-prover.org/ \
		https://github.com/abella-prover/abella

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/LICENSE

BUILD_DEPENDS=	menhir>=20231231:devel/menhir \
		ocaml-dune>=3.7.1_2:devel/ocaml-dune \
		ocaml-cmdliner>=0:devel/ocaml-cmdliner \
		ocaml-yojson>=0:textproc/ocaml-yojson

USES=		gmake

PLIST_FILES=	bin/${PORTNAME}

.include <bsd.port.options.mk>

.if ${ARCH} == powerpc
USE_BINUTILS=	yes
.endif

do-install:
	@${INSTALL_PROGRAM} ${WRKSRC}/_build/default/src/${PORTNAME}.exe ${STAGEDIR}${PREFIX}/bin/${PORTNAME}

.include <bsd.port.mk>
