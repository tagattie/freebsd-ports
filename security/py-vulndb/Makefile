PORTNAME=	vulndb
PORTVERSION=	0.0.19
PORTREVISION=	1
CATEGORIES=	security python
MASTER_SITES=	PYPI
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	dbaio@FreeBSD.org
COMMENT=	Provides access to the vulndb information in Python
WWW=		https://pypi.org/project/vulndb/

LICENSE=	BSD3CLAUSE
LICENSE_FILE=	${WRKSRC}/LICENSE

BUILD_DEPENDS=	${PYTHON_PKGNAMEPREFIX}setuptools-git>=1.1:devel/py-setuptools-git@${PY_FLAVOR}

USES=		python
USE_PYTHON=	autoplist distutils
NO_ARCH=	yes

.include <bsd.port.mk>
