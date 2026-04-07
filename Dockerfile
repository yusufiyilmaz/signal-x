# 1. Aşama: Derleme (Builder)
FROM rust:1.74-slim AS builder
WORKDIR /app
# Kodları kopyala
COPY . .
# Projeyi release modunda (en yüksek performans) derle
RUN cargo build --release

# 2. Aşama: Çalıştırma (Runtime) - Sadece gerekli olanları alıyoruz
FROM debian:bookworm-slim
WORKDIR /app

# Gerekli sistem kütüphanelerini kur (Ağ araçları için)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Derlenmiş binary dosyasını kopyala (Crate adını 'pentester' yapmıştık)
COPY --from=builder /app/target/release/pentester /usr/local/bin/pentester
# Web paneli için statik dosyaları kopyala
COPY static ./static

# Web panel portunu dışa aç
EXPOSE 3000

# Konteyner çalıştığında doğrudan uygulamayı tetikle
ENTRYPOINT ["pentester"]
