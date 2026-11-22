# Pruebas y Evidencias — Chispazo Ledger (`chispazo_ledger`)

## 1. Identificadores y Despliegue en Devnet
- **Red:** Solana Devnet
- **Program ID / Declare ID:** `W9kWQdPdHQLQ3PBES4pYMEfPQKdsMqRSDSv8zCsZQWe`
- **Upgrade Authority / Wallet:** `BKWmbnCm6AWU9rasaWFTLpHNuBWuuBF3e1zmWwP5Rs3R`
- **Cuenta IDL on-chain:** `8jeV9kEFUzRKQRoDFRB4nwwdwGUKnyHCJAPW2VV5Ph2dk`
- **Firma de despliegue (`anchor deploy`):** `4V5Zt72LJTV7oqjUQtBSdRLm4qSJdivdjVdgKEryJr9qXHpJt9zm5UpQkPsWZohxVcLtjBFVFGBo4fVpfguAgeK`
- **Explorador del programa:** `https://explorer.solana.com/address/W9kWQdPdHQLQ3PBES4pYMEfPQKdsMqRSDSv8zCsZQWe?cluster=devnet`
- **Explorador de despliegue:** `https://explorer.solana.com/tx/4V5Zt72LJTV7oqjUQtBSdRLm4qSJdivdjVdgKEryJr9qXHpJt9zm5UpQkPsWZohxVcLtjBFVFGBo4fVpfguAgeK?cluster=devnet`

## 2. Ejecución de Pruebas (`4 passing (3s)`)
- **1. Crea `MerchantConfig` y `PaymentOrder` (PDAs relacionadas):**
  - `sigCreate`: `5wx1dKpiKYsnmgMAmUmoscMnbNtDDePHfRXPTuX4jzpXUzdHeFeiSfcaG1meULbx7nbSW8UamRukmPFFipopWCJk`
  - Explorer: `https://explorer.solana.com/tx/5wx1dKpiKYsnmgMAmUmoscMnbNtDDePHfRXPTuX4jzpXUzdHeFeiSfcaG1meULbx7nbSW8UamRukmPFFipopWCJk?cluster=devnet`
- **2. Rechaza cierre cuando la orden sigue `Pending`:** Aprobado (transacción rechazada como se esperaba).
- **3. Rechaza modificación desde una autoridad no autorizada:** Aprobado (transacción rechazada como se esperaba).
- **4. Modifica estado a `Settled` y cierra la cuenta (cierre condicionado):**
  - `sigSettle`: `39gyBNC6UgKHYxdTVg6YiTG3i55c2Zu7jmmCjhppVhkyisBdzM9jP28F7HQyNSs4dhnwxq3iPcCS4Kfkk1pJLgWB`
  - `sigClose`: `5SNqquNU9tiujtuk9LYDyvBM8gXxN6cX6h3GbQm3i7Ad6rnhAm5utfUqHabJhns27Y2KE5eUZWycHusozPk8vdwd`
  - Explorer (`sigSettle`): `https://explorer.solana.com/tx/39gyBNC6UgKHYxdTVg6YiTG3i55c2Zu7jmmCjhppVhkyisBdzM9jP28F7HQyNSs4dhnwxq3iPcCS4Kfkk1pJLgWB?cluster=devnet`
  - Explorer (`sigClose`): `https://explorer.solana.com/tx/5SNqquNU9tiujtuk9LYDyvBM8gXxN6cX6h3GbQm3i7Ad6rnhAm5utfUqHabJhns27Y2KE5eUZWycHusozPk8vdwd?cluster=devnet`