#!/bin/sh
# Generates the local Postgres's TLS material in /tls (the repository's .local/tls): a CA, and a
# server certificate signed by it. Osiris connects with verify-ca against ca.pem (DB_CA_PEM), the
# same way it connects to Cloud SQL. Run by docker-compose's `postgres-tls`; does nothing when
# the files already exist.
set -eu
cd /tls
if [ -f ca.pem ] && [ -f server.crt ] && [ -f server.key ]; then
  echo "local TLS certificates already exist"
  exit 0
fi
apk add --no-cache openssl >/dev/null
openssl req -x509 -newkey rsa:2048 -nodes -days 3650 -sha256 \
  -subj "/CN=osiris-local-ca" -keyout ca.key -out ca.pem \
  -addext "basicConstraints=critical,CA:TRUE" -addext "keyUsage=critical,keyCertSign,cRLSign"
openssl req -newkey rsa:2048 -nodes -sha256 -subj "/CN=localhost" -keyout server.key -out server.csr
cat > server.ext <<'EXT'
subjectAltName=DNS:localhost,DNS:postgres,IP:127.0.0.1
basicConstraints=critical,CA:FALSE
keyUsage=critical,digitalSignature,keyEncipherment
extendedKeyUsage=serverAuth
EXT
openssl x509 -req -in server.csr -CA ca.pem -CAkey ca.key -CAcreateserial -days 825 -sha256 \
  -extfile server.ext -out server.crt
rm -f server.csr server.ext ca.srl
chmod 644 ca.pem server.crt
chmod 600 ca.key server.key
echo "local TLS certificates generated"
