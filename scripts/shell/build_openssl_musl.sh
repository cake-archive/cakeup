curl https://www.openssl.org/source/old/1.0.2/openssl-1.0.2g.tar.gz -s --output openssl-1.0.2g.tar.gz
tar xzf openssl-1.0.2g.tar.gz

cd openssl-1.0.2g
CC=musl-gcc ./Configure --prefix=$1 no-dso linux-x86_64 -fPIC

echo "Building..."
make install

cd .. 
rm -rf openssl-1.0.2g
rm openssl-1.0.2g.tar.gz