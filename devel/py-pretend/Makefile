PORTNAME=		pretend
PORTVERSION=		1.0.9
PORTREVISION=	1
DISTVERSIONPREFIX=	v
CATEGORIES=		devel python
PKGNAMEPREFIX=		${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	bofh@FreeBSD.org
COMMENT=	Library for stubbing in Python
WWW=		https://github.com/alex/pretend

LICENSE=	BSD3CLAUSE
LICENSE_FILE=	${WRKSRC}/LICENSE.rst

TEST_DEPENDS=	${PYTHON_PKGNAMEPREFIX}pytest>0:devel/py-pytest@${PY_FLAVOR}

USES=		python
USE_PYTHON=	autoplist distutils

NO_ARCH=	yes

USE_GITHUB=	yes
GH_ACCOUNT=	alex

do-test:
	@cd ${WRKSRC} && ${PYTHON_CMD} -m pytest

.include <bsd.port.mk>
