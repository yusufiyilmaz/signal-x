# API Dokumantasyonu

## POST /api/scan

Istek:
`json
{ "target": "192.168.1.1", "start_port": 1, "end_port": 1024, "timeout_ms": 200 }
```

Yanit:
`json
{ "success": true, "target": "192.168.1.1", "open_ports": [], "os_guess": "Windows", "security_score": "A", "report_md": "..." }
```

## POST /api/multiscan

Istek:
`json
{ "targets": ["192.168.1.1","192.168.1.2"], "start_port": 1, "end_port": 1024 }
```

## POST /api/network

Istek:
`json
{ "base_ip": "192.168.1", "start": 1, "end": 254 }
```

## GET /api/health

Yanit: Signal-X calisiyor!
