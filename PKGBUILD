pkgname=blackjack-tui
pkgver=0.1.0
pkgrel=1
pkgdesc='Terminal blackjack game'
arch=('x86_64' 'aarch64')
url='https://example.invalid/blackjack-tui'
license=('MIT')
depends=('gcc-libs')
makedepends=('rust')
source=()
sha256sums=()

build() {
  cd "$srcdir/$pkgname"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname"
  install -Dm755 target/release/bljack "$pkgdir/usr/bin/bljack"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
