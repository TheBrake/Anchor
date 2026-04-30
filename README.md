# My First Anchor Transfer

Este es mi primer programa desarrollado en **Solana** usando el framework **Anchor**. Este programa permite realizar transferencias de SOL de una cuenta a otra utilizando **CPI** (Cross-Program Invocation).

## Tecnologías Que utilice
* Rust
* Anchor Framework
* TypeScript (para los tests)

## Cómo ejecutarlo

1. **Inicia el validador local**:
   solana-test-validator

2. **Compila y despliega**
   anchor build
   anchor deploy

3. **Corre los tests**:
   anchor run test


## Aprendizajes clave
* Uso de `CpiContext` para llamar al System Program, resulta que siempre se debe llamar.
* Manejo de cuentas y firmantes (`Signer`). 
* Configuración de ambiente local con doble terminal.
* Recuerda siempre verificar lib.rs y en /test/.ts ya que la ID es unica en el momento en que creas un proyecto, asi que evita dañar las cosas.
