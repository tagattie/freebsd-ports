PORTNAME=	pycha
PORTVERSION=	0.8.1
PORTREVISION=	2
CATEGORIES=	graphics python
MASTER_SITES=	PYPI
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	sunpoet@FreeBSD.org
COMMENT=	Library for making charts with Python
WWW=		https://bitbucket.org/lgs/pycha

LICENSE=	LGPL3+
LICENSE_FILE=	${WRKSRC}/COPYING

BUILD_DEPENDS=	${PYTHON_PKGNAMEPREFIX}setuptools>=0:devel/py-setuptools@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}wheel>=0:devel/py-wheel@${PY_FLAVOR}
RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}six>=0:devel/py-six@${PY_FLAVOR}

USES=		python
USE_PYTHON=	autoplist concurrent pep517

NO_ARCH=	yes

OPTIONS_DEFINE=	CAIRO
OPTIONS_DEFAULT=CAIRO

CAIRO_RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}cairo>=0:graphics/py-cairo@${PY_FLAVOR}

.include <bsd.port.mk>
