PORTNAME=	Crypt-Primes
PORTVERSION=	0.52
CATEGORIES=	security perl5
MASTER_SITES=	CPAN
PKGNAMEPREFIX=	p5-

MAINTAINER=	perl@FreeBSD.org
COMMENT=	Provable Prime Number Generator suitable for Cryptographic Applications
WWW=		https://metacpan.org/release/Crypt-Primes

LICENSE=	ART10 GPLv1+
LICENSE_COMB=	dual
LICENSE_FILE=	${WRKSRC}/LICENSE

BUILD_DEPENDS=	${RUN_DEPENDS}
RUN_DEPENDS=	p5-Crypt-Random>=0:security/p5-Crypt-Random \
		p5-Math-Pari>=0:math/p5-Math-Pari

USES=		perl5
USE_PERL5=	configure

NO_ARCH=	yes

.include <bsd.port.mk>
