PORTNAME=	pybloom
PORTVERSION=	3.0.0
PORTREVISION=	1
DISTVERSIONPREFIX=	v
CATEGORIES=	math python
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	lwhsu@FreeBSD.org
COMMENT=	Probabilistic data structure
WWW=		https://github.com/joseph-fox/python-bloomfilter

LICENSE=	MIT

RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}bitarray>0:devel/py-bitarray@${PY_FLAVOR}

USES=		python
USE_PYTHON=	distutils autoplist

USE_GITHUB=	yes
GH_ACCOUNT=	joseph-fox
GH_PROJECT=	python-bloomfilter

.include <bsd.port.mk>
