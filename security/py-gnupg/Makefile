PORTNAME=	gnupg
PORTVERSION=	3.1.1
PORTREVISION=	1
CATEGORIES=	security python
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	cs@FreeBSD.org
COMMENT=	Python wrapper for GnuPG
WWW=		https://github.com/isislovecruft/python-gnupg

LICENSE=	GPLv3

RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}psutil>=0.5.1:sysutils/py-psutil@${PY_FLAVOR}

USES=		python
USE_GITHUB=	yes
USE_PYTHON=	distutils autoplist

GH_ACCOUNT=	isislovecruft
GH_PROJECT=	python-gnupg

.include <bsd.port.mk>
