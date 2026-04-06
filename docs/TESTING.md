# Test Rehberi

## Testleri Calistir

cargo test
cargo test -p pentester

## Test Listesi

| Test | Modul | Aciklama |
|------|-------|----------|
| test_get_service_name | scanner | Port servis adi eslesmesi |
| test_parse_version_ssh | scanner | SSH banner parse |
| test_parse_version_redis | scanner | Redis banner tespiti |
| test_parse_version_empty | scanner | Bos banner isleme |
| test_port_status_display | scanner | Port durum string |
| test_port_status_filtered_on_timeout | scanner | Filtreli port |
| test_scan_port_closed | scanner | Kapali port tarama |
| test_banner_empty_on_closed | scanner | Kapali portta banner |
| test_security_score_a | report | Bos liste A notu |
| test_security_score_f | report | Cok port F notu |
| test_security_score_riskli_port | report | Riskli port etkisi |
| test_generate_markdown | report | Markdown rapor icerigi |
