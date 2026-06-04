# Objetivo: baseline ETSI 014 + SKIP com preparacao para Cisco

## 1. Objetivo do teste

O objetivo do ambiente e demonstrar, de forma executavel e local, que:

```text
Alice e Bob conseguem obter material de PPK equivalente
a partir de duas instancias logicas ETSI GS QKD 014,
por meio de Key Providers independentes,
e expor esse material via SKIP para uso futuro em IKEv2/RFC8784.
```

Nesta baseline, o "QKD" nao e um simulador fisico. O projeto nao modela canal
quantico, BB84, NetSquid, QBER, reconciliacao ou privacy amplification.

O que a baseline testa e a entrega logica de material de chave por um
subconjunto fiel do ETSI GS QKD 014, usando o KMS Experimental
Key-Management Simulator em `kms/` como fonte oficial das duas KMEs locais.

## 2. Fontes de autoridade

Este documento deve permanecer consistente com:

- `baseline.md`
- `docs/architecture.md`
- `docs/etsi014-alignment.md`
- `docs/api-etsi014-mock.md`
- `docs/api-skip.md`
- `docs/data-model.md`
- `docs/security-assumptions.md`
- `kms/README.md`
- `kms/adrs/0001-experimental-key-management-simulator.md`
- `kms/adrs/0002-etsi-014-protocol-fidelity.md`
- `kms/adrs/0003-sae-identity-and-topology-policy.md`
- `kms/adrs/0004-key-source-and-etsi-storage-lifecycle.md`
- `kms/adrs/0005-testing-and-fidelity-guidelines.md`

Autoridade externa:

- ETSI GS QKD 014 para a fronteira KME/SAE.
- `draft-singh-skip-00` para a fronteira Key Provider/encryptor.
- RFC 8784 apenas como contexto futuro para PPK em IKEv2.
- Roteadores Cisco apenas como consumidor futuro de SKIP/RFC8784, nao como
  funcionalidade validada nesta baseline.

## 3. Hipotese tecnica

A hipotese do teste e:

```text
Se duas KMEs locais implementam o subconjunto ETSI GS QKD 014 adotado pelo
kms/ e conhecem os mesmos bytes de chave para o mesmo key_ID, entao dois
Key Providers independentes podem atuar como SAE-A e SAE-B, coletar a chave
em suas KMEs locais, converter ETSI base64 para SKIP hex e entregar o mesmo
material de PPK para Alice e Bob via SKIP.
```

Essa hipotese separa claramente:

| Area | Status na baseline |
|---|---|
| ETSI 014 logico | Dentro do escopo. |
| `kms/` como KME experimental | Dentro do escopo. |
| SKIP para encryptors simulados | Dentro do escopo. |
| RFC 8784/IKEv2 real | Futuro. |
| Roteadores Cisco reais | Futuro. |
| QKD fisico | Fora do escopo. |
| Producao | Fora do escopo. |

## 4. Decisao central de arquitetura

A baseline usa duas instancias separadas do `kms/`, nao uma KME generica e nao
um servico KME Python paralelo.

```text
KME-A = instancia do kms/ com KME_ID=KME-A e SAE local SAE-A
KME-B = instancia do kms/ com KME_ID=KME-B e SAE local SAE-B
```

`KeyProvider-A` atua como `SAE-A` e chama somente `KME-A`.
`KeyProvider-B` atua como `SAE-B` e chama somente `KME-B`.

Cada Key Provider possui seu proprio SQLite. Nao existe banco central entre
providers. Cada KME tambem possui storage separado. A unica sincronizacao
permitida na baseline e uma fake key-source deterministica, configurada nos
dois lados, para que o mesmo ETSI `key_ID` gere ou resolva os mesmos bytes.

## 5. Arquitetura logica

```text
                 ETSI GS QKD 014 subset                      SKIP subset

  +-----------------------------+                     +----------------------+
  | KME-A                       |                     | Encryptor-A sim      |
  | kms simulator instance      |                     | localSystemID=Alice  |
  | KME_ID=KME-A                |                     +----------+-----------+
  | connected SAE: SAE-A        |                                |
  | independent KME storage     |                                | GET /key?remoteSystemID=Bob
  +--------------+--------------+                                v
                 ^                                  +-------------+-------------+
                 | status, enc_keys                 | KeyProvider-A             |
                 | caller SAE: SAE-A                | ETSI client identity SAE-A|
                 | peer SAE: SAE-B                  | SKIP server for Alice     |
                 |                                  | independent SQLite DB     |
                 |                                  +-------------+-------------+
                 |                                                |
                 |                                                | keyId handoff
                 |                                                v
                 |                                  +-------------+-------------+
                 |                                  | KeyProvider-B             |
                 |                                  | ETSI client identity SAE-B|
                 |                                  | SKIP server for Bob       |
                 |                                  | independent SQLite DB     |
                 |                                  +-------------+-------------+
                 |                                                ^
                 v                                                | GET /key/{keyId}?remoteSystemID=Alice
  +--------------+--------------+                                |
  | KME-B                       |                     +----------+-----------+
  | kms simulator instance      |                     | Encryptor-B sim      |
  | KME_ID=KME-B                |                     | localSystemID=Bob    |
  | connected SAE: SAE-B        |                     +----------------------+
  | independent KME storage     |
  +-----------------------------+
```

O handoff de `keyId` entre encryptors simulados representa, no laboratorio, o
futuro transporte de uma identidade de PPK. Ele nao implementa IKEv2, IPsec ou
RFC 8784 real.

## 6. Papeis dos componentes

| Componente | Papel | Limite |
|---|---|---|
| `kms/` | Simulador Rust oficial da baseline para KME-A e KME-B. | Nao e KMS de producao. |
| `KME-A` | Servidor ETSI 014 local de `SAE-A`. | Nao fala SKIP. |
| `KME-B` | Servidor ETSI 014 local de `SAE-B`. | Nao fala SKIP. |
| `KeyProvider-A` | Cliente ETSI como `SAE-A` e servidor SKIP para Alice. | Nao chama `KME-B`. |
| `KeyProvider-B` | Cliente ETSI como `SAE-B` e servidor SKIP para Bob. | Nao chama `KME-A`. |
| `Encryptor-A sim` | Cliente SKIP que solicita chave nova para Bob. | Nao e SAE ETSI. |
| `Encryptor-B sim` | Cliente SKIP que recupera chave por `keyId`. | Nao e Cisco real. |
| Roteadores Cisco | Consumidores futuros de SKIP/RFC8784. | Fora da validacao atual. |

## 7. Contrato ETSI 014 adotado pelo `kms/`

O objetivo nao deve usar nomes de conveniencia para a API ETSI. O subconjunto
publico aceito e:

| Metodo | Path | Uso |
|---|---|---|
| `GET` | `/api/v1/keys/{slave_SAE_ID}/status` | Status/topologia para o slave SAE solicitado. |
| `POST` | `/api/v1/keys/{slave_SAE_ID}/enc_keys` | Requisicao completa de chaves de cifragem. |
| `GET` | `/api/v1/keys/{slave_SAE_ID}/enc_keys` | Forma simples de requisicao de chaves. |
| `POST` | `/api/v1/keys/{master_SAE_ID}/dec_keys` | Recuperacao completa por IDs de chave. |
| `GET` | `/api/v1/keys/{master_SAE_ID}/dec_keys` | Forma simples de recuperacao por `key_ID`. |

Nomes JSON publicos devem preservar o ETSI:

- `source_KME_ID`
- `target_KME_ID`
- `master_SAE_ID`
- `slave_SAE_ID`
- `key_ID`
- `key_IDs`
- `additional_slave_SAE_IDs`
- `extension_mandatory`
- `extension_optional`
- `status_extension`
- `key_container_extension`

O material de chave na fronteira ETSI e base64. O material de chave na
fronteira SKIP e hexadecimal.

## 8. Identidade SAE e autorizacao

Topologia SAE/KME e politica de autorizacao, nao apenas metadata.

| Modo | Fonte da identidade SAE |
|---|---|
| mTLS habilitado | Common Name do certificado de cliente autenticado |
| mTLS desabilitado | Header local `x-sae-id` |

`x-sae-id` so e valido em HTTP local sem mTLS. Em modo mTLS, o caller SAE vem
do certificado.

Antes de liberar chave, a KME deve rejeitar:

- caller SAE desconhecido;
- peer SAE desconhecido;
- master incorreto;
- slave incorreto;
- ownership divergente;
- chave inexistente;
- chave ja consumida;
- tamanho de chave nao suportado;
- `extension_mandatory` nao suportado.

## 9. `keyId` SKIP deterministico

O identificador SKIP da baseline e:

```text
skip_key_id = SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}
```

Exemplo:

```text
key_ID      = QKD-000001
master SAE  = SAE-A
slave SAE   = SAE-B
skip_key_id = SKIP-SAE-A-SAE-B-QKD-000001
```

Esse identificador nao contem material de chave. Ele existe para handoff,
debug e resolucao deterministica no laboratorio.

### 9.1 Contrato SKIP que prepara a fase Cisco

Os Key Providers devem expor o subconjunto SKIP documentado em
`docs/api-skip.md`:

| Metodo | Path | Uso |
|---|---|---|
| `GET` | `/capabilities` | Publica capacidades e sistemas locais/remotos. |
| `GET` | `/key?remoteSystemID={id}` | Entrega uma chave nova para o sistema remoto. |
| `GET` | `/key?remoteSystemID={id}&size={bits}` | Entrega chave nova com tamanho solicitado. |
| `GET` | `/key/{keyId}?remoteSystemID={id}` | Recupera chave existente pelo `keyId`. |
| `GET` | `/entropy` | Entrega entropia conforme contrato SKIP. |
| `GET` | `/entropy?minentropy={bits}` | Entrega entropia minima solicitada. |

Campos publicos relevantes:

- `localSystemID`
- `remoteSystemID`
- `keyId`
- `key`
- `entropy`

Para a fase Cisco, o ponto critico e que o `key` retornado por SKIP seja
hexadecimal e que `keyId` seja resolvivel nos dois lados sem banco central
entre Key Providers.

## 10. Fluxo ponta a ponta

### 10.1 KeyProvider-A consulta status

```http
GET /api/v1/keys/SAE-B/status
x-sae-id: SAE-A
```

Resposta esperada:

```json
{
  "source_KME_ID": "KME-A",
  "target_KME_ID": "KME-B",
  "master_SAE_ID": "SAE-A",
  "slave_SAE_ID": "SAE-B",
  "key_size": 256,
  "stored_key_count": 1000000,
  "max_key_count": 1000000,
  "max_key_per_request": 128,
  "max_key_size": 256,
  "min_key_size": 256,
  "max_SAE_ID_count": 0,
  "status_extension": {}
}
```

### 10.2 KeyProvider-A solicita `enc_keys`

```http
POST /api/v1/keys/SAE-B/enc_keys
content-type: application/json
x-sae-id: SAE-A
```

```json
{ "number": 1, "size": 256 }
```

Resposta ETSI:

```json
{
  "keys": [
    {
      "key_ID": "QKD-000001",
      "key": "<base64>"
    }
  ],
  "key_container_extension": {}
}
```

`KME-A` registra a chave com:

```text
master_sae_id = SAE-A
slave_sae_id  = SAE-B
key_ID        = QKD-000001
```

### 10.3 KeyProvider-A entrega SKIP para Alice

O provider converte:

```text
ETSI base64 -> bytes -> SKIP hex
```

`Encryptor-A sim` chama:

```http
GET /key?remoteSystemID=Bob
```

Resposta SKIP:

```json
{
  "keyId": "SKIP-SAE-A-SAE-B-QKD-000001",
  "key": "<hex>"
}
```

### 10.4 Handoff de `keyId`

`Encryptor-A sim` passa apenas o `keyId` para `Encryptor-B sim`.

Na futura integracao com roteadores Cisco, esse ponto deve corresponder ao
conceito de identidade de PPK transportada pela negociacao IKEv2/RFC8784. A
baseline atual so simula esse handoff.

### 10.5 KeyProvider-B recupera `dec_keys`

`Encryptor-B sim` chama:

```http
GET /key/SKIP-SAE-A-SAE-B-QKD-000001?remoteSystemID=Alice
```

`KeyProvider-B` resolve:

```text
SKIP-SAE-A-SAE-B-QKD-000001 -> QKD-000001
```

Depois chama `KME-B` como `SAE-B`:

```http
POST /api/v1/keys/SAE-A/dec_keys
content-type: application/json
x-sae-id: SAE-B
```

```json
{
  "key_IDs": [
    { "key_ID": "QKD-000001" }
  ]
}
```

Resposta ETSI:

```json
{
  "keys": [
    {
      "key_ID": "QKD-000001",
      "key": "<base64>"
    }
  ],
  "key_container_extension": {}
}
```

`KeyProvider-B` converte base64 para hex e responde:

```json
{
  "keyId": "SKIP-SAE-A-SAE-B-QKD-000001",
  "key": "<hex>"
}
```

Invariante final:

```text
key_hex_alice == key_hex_bob
```

Depois de `dec_keys` bem-sucedido, `KME-B` consome `QKD-000001`. Uma segunda
recuperacao do mesmo `key_ID` deve falhar.

## 11. Preparacao para roteadores Cisco

Roteadores Cisco nao fazem parte da primeira validacao. Eles sao a fronteira
de integracao futura depois que SKIP e o fluxo ETSI/KMS estiverem estaveis.

A arquitetura deve, desde o inicio, deixar os Key Providers prontos para serem
substitutos dos encryptors simulados como fonte SKIP para roteadores:

```text
                 Rede de gerenciamento SKIP/HTTPS

  +-------------------+                         +-------------------+
  | KeyProvider-A     |                         | KeyProvider-B     |
  | SKIP HTTPS        |                         | SKIP HTTPS        |
  | localSystemID=A   |                         | localSystemID=B   |
  +---------+---------+                         +---------+---------+
            ^                                             ^
            | SKIP                                        | SKIP
            |                                             |
  +---------+---------+       IPsec/IKEv2 futuro  +--------+----------+
  | Cisco Router A    |-------------------------->| Cisco Router B    |
  | PPK consumer      |                           | PPK consumer      |
  +-------------------+                           +-------------------+
```

Regras para essa preparacao:

- O roteador Cisco deve enxergar apenas o endpoint SKIP do Key Provider local.
- O roteador Cisco nao deve chamar `KME-A` nem `KME-B`.
- O Key Provider continua sendo o SAE ETSI diante da KME.
- A chave retornada por SKIP deve ser hexadecimal.
- O `keyId` deve ser estavel e resolvivel pelo provider remoto.
- O handoff de `keyId` deve ser compativel com a futura identidade de PPK.
- HTTPS deve ser tratado como requisito para a fase Cisco.
- mTLS ETSI e autenticacao SKIP devem ser validadas antes de qualquer claim de
  seguranca.

Um alvo provavel para laboratorio e Cisco Catalyst 8000V ou uma plataforma
Cisco que suporte o modo de PPK dinamica via SKIP. A fase Cisco deve confirmar
versao, licenciamento, comandos, formato de chave, tamanho aceito e semantica
de renovacao antes de documentar uma configuracao final.

## 12. Criterios de prontidao para Cisco

A integracao com roteadores Cisco so deve comecar depois que estes criterios
passarem com encryptors simulados:

```text
1. GET /capabilities funciona nos dois Key Providers.
2. GET /key?remoteSystemID=Bob retorna keyId e key em hexadecimal.
3. GET /key/{keyId}?remoteSystemID=Alice retorna a mesma key.
4. keyId usa SKIP-SAE-A-SAE-B-QKD-000001 para o fluxo A->B.
5. KeyProvider-A chama somente KME-A.
6. KeyProvider-B chama somente KME-B.
7. dec_keys e one-time.
8. Repetir dec_keys com o mesmo key_ID falha.
9. key_hex_alice == key_hex_bob.
10. Nenhum log contem chave completa.
11. Os providers rodam com endpoints HTTPS para o perfil Cisco.
12. localSystemID e remoteSystemID estao configurados de forma estavel.
```

## 13. Runtime de laboratorio

O runtime primario e Docker Compose.

Servicos da baseline:

```text
kme-a
kme-b
keyprovider-a
keyprovider-b
encryptor-a-sim
encryptor-b-sim
```

Tecnologias:

| Area | Escolha |
|---|---|
| KME | Rust `kms/` simulator |
| Key Providers | Python/FastAPI em fases futuras |
| Encryptors simulados | Python/FastAPI ou cliente HTTP simples |
| Provider DB | SQLite separado por provider |
| KME storage | Storage separado por instancia |
| Orquestracao | Docker Compose |
| Testes | `cargo test`, `pytest`, testes e2e |

Redes locais sugeridas:

```text
etsi_net: KME-A, KME-B, KeyProvider-A, KeyProvider-B
skip_net: KeyProvider-A, KeyProvider-B, encryptor-a-sim, encryptor-b-sim
mgmt_net: test-runner, observability, perfis futuros Cisco
```

Para a fase Cisco, uma rede de gerenciamento dedicada deve expor apenas SKIP
HTTPS dos providers para os roteadores. A rede ETSI entre providers e KMEs deve
permanecer separada.

## 14. Modelo de dados esperado

Dominios de identificadores:

| Identificador | Exemplo | Uso |
|---|---|---|
| KME ID | `KME-A`, `KME-B` | Topologia ETSI. |
| SAE ID | `SAE-A`, `SAE-B` | Identidade ETSI. |
| ETSI `key_ID` | `QKD-000001` | Registro de chave na KME. |
| SKIP `keyId` | `SKIP-SAE-A-SAE-B-QKD-000001` | Handoff para encryptor/Cisco. |
| `localSystemID` | `Alice`, `Bob` | Rotulo SKIP local. |
| `remoteSystemID` | `Bob`, `Alice` | Rotulo SKIP remoto. |

Storage ETSI por KME:

```text
key_ID
master_sae_id
slave_sae_id
key bytes
availability / consumption state
```

Storage do provider:

```text
skip_key_id
key_ID
master_sae_id
slave_sae_id
localSystemID
remoteSystemID
fingerprint
lifecycle status
```

Nunca registrar chave completa em logs. Use fingerprint curto para depuracao.

## 15. Testes obrigatorios

### 15.1 Verificacao do `kms/`

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

### 15.2 Testes ETSI/KME

```text
test_exact_etsi_paths_and_methods()
test_status_uses_etsi_json_names()
test_enc_keys_returns_base64_key_container()
test_dec_keys_retrieves_by_key_ID()
test_dec_keys_consumes_successful_key()
test_unknown_sae_is_rejected()
test_wrong_master_or_slave_is_rejected()
test_unsupported_key_size_is_rejected()
```

### 15.3 Testes SKIP/provider

```text
test_capabilities_returns_local_and_remote_systems()
test_skip_key_endpoint_returns_keyId_and_hex_key()
test_skip_keyId_maps_to_etsi_key_ID()
test_provider_a_calls_only_kme_a()
test_provider_b_calls_only_kme_b()
test_providers_use_separate_sqlite_state()
test_no_full_key_material_in_logs()
```

### 15.4 Teste e2e principal

```text
test_e2e_alice_and_bob_receive_same_ppk_material()
```

Fluxo esperado:

```text
1. KeyProvider-A chama KME-A como SAE-A.
2. KME-A emite enc_keys para SAE-B.
3. KeyProvider-A entrega keyId e key hex para Alice via SKIP.
4. Alice entrega keyId para Bob por handoff simulado.
5. Bob chama KeyProvider-B com keyId.
6. KeyProvider-B chama KME-B como SAE-B usando dec_keys.
7. KME-B retorna os mesmos bytes em base64.
8. KeyProvider-B entrega key hex para Bob.
9. O teste valida key_hex_alice == key_hex_bob.
10. O teste valida que repetir dec_keys falha.
```

### 15.5 Testes de prontidao Cisco

Antes de configurar roteadores Cisco reais:

```text
test_skip_https_profile_is_enabled()
test_key_is_hex_and_expected_size()
test_keyId_is_stable_for_remote_lookup()
test_remote_lookup_works_without_remote_provider_db_sharing()
test_cisco_profile_does_not_expose_etsi_network_to_router()
test_provider_logs_use_fingerprints_only()
```

## 16. Metricas do baseline

Metricas uteis desde o inicio:

```text
etsi_status_latency_ms
etsi_enc_keys_latency_ms
etsi_dec_keys_latency_ms
skip_get_key_latency_ms
skip_get_key_by_id_latency_ms
ppk_match_success_count
ppk_mismatch_count
dec_keys_reuse_rejection_count
unknown_sae_rejection_count
wrong_ownership_rejection_count
```

Essas metricas podem apoiar comparacoes futuras entre:

```text
PPK estatica
PPK via ETSI 014 subset + SKIP
PPK via KME experimental com fake key-source deterministica
PPK via QKD real
```

## 17. Ordem recomendada de implementacao

1. Manter `baseline.md`, arquitetura e contratos alinhados aos ADRs do `kms/`.
2. Evoluir `kms/` para o subconjunto ETSI 014, se houver gaps de runtime.
3. Implementar KeyProvider-A e KeyProvider-B como clientes ETSI e servidores
   SKIP.
4. Garantir SQLite separado por provider.
5. Implementar encryptors simulados.
6. Implementar testes unitarios, router tests, integracao e e2e.
7. Validar o fluxo `key_hex_alice == key_hex_bob`.
8. Adicionar perfil HTTPS/SKIP para preparacao Cisco.
9. Documentar a configuracao Cisco somente depois de confirmar plataforma,
   versao, licenciamento e comportamento operacional.
10. Integrar roteadores Cisco reais em uma fase separada.

## 18. Resultado esperado

Ao final da baseline, o resultado esperado e:

```text
[OK] KME-A e uma instancia do kms/
[OK] KME-B e uma instancia do kms/
[OK] KeyProvider-A atua como SAE-A e chama somente KME-A
[OK] KeyProvider-B atua como SAE-B e chama somente KME-B
[OK] ETSI enc_keys retorna key_ID e key em base64
[OK] SKIP retorna keyId e key em hexadecimal
[OK] KeyProvider-B resolve keyId para key_ID
[OK] ETSI dec_keys e one-time
[OK] PPK Alice == PPK Bob
[OK] Baseline pronta para uma fase Cisco-adjacent
```

## 19. Prompt resumido para futuras sessoes

```text
Quero desenvolver um baseline local que integra duas instancias do kms/
como KMEs ETSI GS QKD 014, dois Key Providers independentes como SAE-A e
SAE-B, e dois encryptors simulados via SKIP. O objetivo e provar que Alice
e Bob recebem o mesmo material de PPK, com ETSI key em base64, SKIP key em
hexadecimal, keyId deterministico no formato
SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}, dec_keys one-time e sem banco
central entre providers.

Nao implementar Cisco real, IKEv2 ou IPsec nesta baseline. Preparar a
arquitetura para uma fase futura com roteadores Cisco como consumidores SKIP,
mantendo Cisco fora da rede ETSI e validando primeiro todos os criterios de
prontidao com encryptors simulados.
```
