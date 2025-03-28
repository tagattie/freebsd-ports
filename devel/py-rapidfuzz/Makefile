PORTNAME=	rapidfuzz
DISTVERSION=	3.10.1
PORTREVISION=	1
CATEGORIES=	devel
MASTER_SITES=	PYPI
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

PATCH_SITES=	https://github.com/${PORTNAME}/RapidFuzz/commit/
PATCHFILES=	d23b79f5fceea0cc04fc38f830ad54ca782e776d.patch:-p1 # https://github.com/rapidfuzz/RapidFuzz/pull/433

MAINTAINER=	yuri@FreeBSD.org
COMMENT=	Rapid fuzzy string matching in Python
WWW=		https://github.com/rapidfuzz/RapidFuzz

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/LICENSE

BUILD_DEPENDS=	cmake:devel/cmake-core \
		ninja:devel/ninja \
		rapidfuzz-cpp>0:devel/rapidfuzz-cpp \
		taskflow>0:devel/taskflow \
		${PYNUMPY} \
		${PYTHON_PKGNAMEPREFIX}rapidfuzz-capi>=1.0.5:devel/py-rapidfuzz-capi@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}scikit-build-core>=0.10.7:devel/py-scikit-build-core@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}scikit-build>=0.13.0:devel/py-scikit-build@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}wheel>0:devel/py-wheel@${PY_FLAVOR}
RUN_DEPENDS=	${PYNUMPY}
TEST_DEPENDS=	${PYTHON_PKGNAMEPREFIX}hypothesis>0:devel/py-hypothesis@${PY_FLAVOR}

USES=		compiler:c++14-lang python
USE_PYTHON=	pep517 autoplist pytest # 1 test fails, see https://github.com/rapidfuzz/RapidFuzz/issues/372

TEST_ENV=	${MAKE_ENV} PYTHONPATH=${STAGEDIR}${PYTHONPREFIX_SITELIBDIR}

post-install:
	@cd ${STAGEDIR}${PYTHON_SITELIBDIR}/${PORTNAME} && \
		${STRIP_CMD} \
			${STAGEDIR}${PYTHON_SITELIBDIR}/${PORTNAME}/*.so \
			${STAGEDIR}${PYTHON_SITELIBDIR}/${PORTNAME}/distance/*.so
	${PYTHON_CMD} -m compileall -d ${PYTHON_SITELIBDIR} ${STAGEDIR}${PYTHON_SITELIBDIR}
	${PYTHON_CMD} -O -m compileall -d ${PYTHON_SITELIBDIR} ${STAGEDIR}${PYTHON_SITELIBDIR}

.include <bsd.port.mk>
