PORTNAME=	ut
DISTVERSIONPREFIX=	v
DISTVERSION=	2.3.1
CATEGORIES=	devel

MAINTAINER=	yuri@FreeBSD.org
COMMENT=	C++20 micro/Unit Testing Framework
WWW=		https://boost-ext.github.io/ut/ \
		https://github.com/boost-ext/ut

LICENSE=	BSL
LICENSE_FILE=	${WRKSRC}/LICENSE.md

USES=		cmake:testing

USE_GITHUB=	yes
GH_ACCOUNT=	boost-ext

CMAKE_OFF=	BOOST_UT_ALLOW_CPM_USE \
		BOOST_UT_BUILD_TESTS \
		BOOST_UT_BUILD_EXAMPLES
CMAKE_ARGS=	-DFREEBSD_WRKSRC_cpm=${WRKSRC_cpm} \
		-DCURRENT_CPM_VERSION=0.38.7
CMAKE_TESTING_ON=	BOOST_UT_BUILD_TESTS # 7 tests fail, see https://github.com/boost-ext/ut/issues/600

PLIST_FILES=	include/boost/ut.hpp \
		include/boost/ut.cppm \
		lib/cmake/ut/utConfig.cmake \
		lib/cmake/ut/utConfigVersion.cmake

NO_ARCH=	yes

.include <bsd.port.mk>
