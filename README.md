# Effect Smart

Editor de motion graphics, animações e efeitos visuais para Windows e Linux.

## O que o programa faz

- Criar projetos de composição com camadas, formas e textos.
- Organizar animações com quadros-chave (keyframes) e linha do tempo.
- Trabalhar com máscaras, efeitos e composição por GPU.
- Importar materiais, ajustar propriedades e exportar o resultado.

## Como usar

1. Abra o Effect Smart e crie um projeto.
2. Importe vídeos, imagens ou áudios para o projeto.
3. Organize as camadas e defina a duração da composição.
4. Use os quadros-chave para animar posição, escala, rotação e outros parâmetros.
5. Adicione máscaras, efeitos e ajustes.
6. Pré-visualize o resultado e utilize as opções de exportação disponíveis.

Alguns formatos de exportação exigem codecs e ferramentas adicionais do sistema.

## Instalação

O código pode ser compilado com Rust 1.95 e as dependências gráficas necessárias.
Os arquivos de instalação gerados pelo GitHub Actions são disponibilizados como artifacts ou em **Releases**.

**Windows:** execute o MSI. **Linux:** use AppImage, DEB ou RPM conforme sua distribuição.

## Compilar a partir do código-fonte

```bash
cargo build --release -p effectcraft
```

O executável interno continua chamado `effectcraft` por compatibilidade com o código original. O nome exibido para o público é Effect Smart.

## Instaladores e publicação

Na aba **Actions** do repositório escolha **Gerar MSI Effect Smart**, **Gerar Linux Effect Smart** ou **Publicar Effect Smart**.
Para uma Release, use a versão definida em `[workspace.package]` no `Cargo.toml`, com pontos (por exemplo `0.10.0`). O processo requer compilação bem-sucedida nos dois sistemas.

A publicação em Releases não atualiza automaticamente uma instalação existente. O usuário deverá instalar a nova versão.

## Licenças

Este projeto é derivado de um projeto de código aberto. Os avisos de licença, direitos autorais e atribuições existentes no código-fonte foram preservados nos arquivos correspondentes.
