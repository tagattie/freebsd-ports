PORTNAME=	paperless
PORTVERSION=	2.7.0
PORTREVISION=	20
CATEGORIES=	deskutils python
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

PATCH_SITES=	https://github.com/${GH_ACCOUNT}/${GH_PROJECT}/commit/
PATCHFILES+=	b122931185fb.patch:-p1 # https://github.com/the-paperless-project/paperless/pull/712
PATCHFILES+=	b131c97d1d85.patch:-p1 # https://github.com/the-paperless-project/paperless/pull/716

MAINTAINER=	grembo@FreeBSD.org
COMMENT=	Index and archive scanned paper documents
WWW=		https://paperless.readthedocs.io

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/LICENSE

BUILD_DEPENDS=	${PYTHON_PKGNAMEPREFIX}dateparser>=1.1.0:devel/py-dateparser@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}django-cors-headers>=2.4.0:www/py-django-cors-headers@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}django-crispy-forms>=1.7.2:www/py-django-crispy-forms@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}django-extensions>=2.1.6:www/py-django-extensions@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}django-filter>=2.0.0:www/py-django-filter@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}djangoql>=0.17.0:www/py-djangoql@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}djangorestframework>=3.9.0:www/py-djangorestframework@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}fuzzywuzzy>=0.15.1:devel/py-fuzzywuzzy@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}langdetect>=1.0.9:textproc/py-langdetect@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}pdftotext>=2.1.1:textproc/py-pdftotext@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}pyocr>=0.5.3:graphics/py-pyocr@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}python-dotenv>=0.10.1:www/py-python-dotenv@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}python-gnupg>=0.4.3:security/py-python-gnupg@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}python-magic>=0.4.10:devel/py-python-magic@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}sqlite3>0:databases/py-sqlite3@${PY_FLAVOR}
RUN_DEPENDS:=	${BUILD_DEPENDS}
RUN_DEPENDS+=	gpg2:security/gnupg \
		optipng:graphics/optipng \
		tesseract:graphics/tesseract \
		unpaper:graphics/unpaper

USES=		python shebangfix
USE_GITHUB=	yes
GH_ACCOUNT=	the-paperless-project
USE_RC_SUBR=	paperless-consumer

SHEBANG_FILES=	${WRKSRC}/src/manage.py

CONFLICTS_INSTALL=	${PYTHON_PKGNAMEPREFIX}*paperless-ngx

SUB_FILES=	README nginx.conf paperless.7 pkg-message sshd_config uwsgi.ini
SUB_LIST=	PKGBASE=${PKGBASE} PYTHON_CMD=${PYTHON_CMD} \
		PYTHONPREFIX_SITELIBDIR=${PYTHONPREFIX_SITELIBDIR} \
		ECHO=${ECHO} EGREP=${EGREP} TOUCH=${TOUCH} \
		CHOWN=${CHOWN} RM=${RM:Q}

NO_ARCH=	yes

USERS=		paperless
GROUPS=		paperless

OPTIONS_DEFINE=		DOCS EXAMPLES
OPTIONS_SINGLE=		IMAGICK
OPTIONS_SINGLE_IMAGICK=	IMAGICK6_NOX11 IMAGICK6_X11 IMAGICK7_NOX11 IMAGICK7_X11
OPTIONS_DEFAULT=	IMAGICK7_X11

IMAGICK_DESC=		${IMAGEMAGICK_DESC}
IMAGICK6_NOX11_DESC=	ImageMagick6 without X11 support
IMAGICK6_X11_DESC=	ImageMagick6 with X11 support
IMAGICK7_NOX11_DESC=	ImageMagick7 without X11 support
IMAGICK7_X11_DESC=	ImageMagick7 with X11 support

DOCS_BUILD_DEPENDS=	${PYTHON_PKGNAMEPREFIX}sphinx>=1.8.3,1:textproc/py-sphinx@${PY_FLAVOR}
DOCS_USES=		gmake

IMAGICK6_X11_USES=		magick:6,run
IMAGICK6_NOX11_USES=		magick:6,run,nox11
IMAGICK7_X11_USES=		magick:7,run
IMAGICK7_NOX11_USES=		magick:7,run,nox11

PORTDOCS=	*

PAPERLESS_STATICDIR=	${WRKSRC}/static

do-build:
	@${PYTHON_CMD} ${PYTHON_LIBDIR}/compileall.py \
		-d ${PYTHONPREFIX_SITELIBDIR} \
		-f ${WRKSRC}/src
	@${PYTHON_CMD} -O ${PYTHON_LIBDIR}/compileall.py \
		-d ${PYTHONPREFIX_SITELIBDIR} \
		-f ${WRKSRC}/src
	@${RM} -r ${PAPERLESS_STATICDIR}
	@${SETENV} \
		PAPERLESS_STATICDIR=${PAPERLESS_STATICDIR} \
		${PYTHON_CMD} ${WRKSRC}/src/manage.py collectstatic

do-build-DOCS-on:
	cd ${WRKSRC}/docs && ${SETENV} ${MAKE_ENV} \
		${MAKE_CMD} ${MAKE_ARGS} html SPHINXBUILD=sphinx-build-${PYTHON_VER}

post-build-DOCS-on:
	@${RM} ${WRKSRC}/docs/_build/html/.buildinfo

do-install:
	@${MKDIR} ${STAGEDIR}${PYTHON_SITELIBDIR}/paperless
	@cd ${WRKSRC}/resources && \
		${COPYTREE_SHARE} . ${STAGEDIR}${DATADIR}
	@cd ${WRKSRC}/src && \
		${COPYTREE_SHARE} . ${STAGEDIR}${PYTHON_SITELIBDIR}/paperless
	@${FIND} ${STAGEDIR}${PYTHON_SITELIBDIR} -name '*.orig' -delete
	@${RLN} ${STAGEDIR}${PYTHON_SITELIBDIR}/paperless/manage.py \
		${STAGEDIR}${PREFIX}/bin/paperless
	@${CHMOD} +x ${STAGEDIR}${PREFIX}/bin/paperless
	@${MKDIR} ${STAGEDIR}${WWWDIR}
	@cd ${WRKSRC} && \
		${COPYTREE_SHARE} static ${STAGEDIR}${WWWDIR}
	@${SED} -e 's|%%WWWDIR%%|${WWWDIR}|g;' \
		${WRKSRC}/paperless.conf.example \
		> ${STAGEDIR}${PREFIX}/etc/paperless.conf.sample
	@${MKDIR} ${STAGEDIR}/var/db/paperless/consume/input \
		${STAGEDIR}/var/db/paperless/media \
		${STAGEDIR}/var/db/paperless/sqlite
	${INSTALL_MAN} ${WRKDIR}/paperless.7 ${STAGEDIR}${PREFIX}/share/man/man7

post-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}/presentation
	@cd ${WRKSRC}/docs/_build/html && \
		${COPYTREE_SHARE} . ${STAGEDIR}${DOCSDIR}
	@cd ${WRKSRC}/presentation && \
		${COPYTREE_SHARE} . ${STAGEDIR}${DOCSDIR}/presentation

post-install-EXAMPLES-on:
	@${MKDIR} ${STAGEDIR}${EXAMPLESDIR}
	${INSTALL_DATA} \
		${WRKDIR}/README \
		${WRKDIR}/nginx.conf \
		${WRKDIR}/sshd_config \
		${WRKDIR}/uwsgi.ini \
		${STAGEDIR}${EXAMPLESDIR}

.include <bsd.port.mk>
