# Sprecher
Content management for Sprecher

## Installation
```sh
git clone https://github.com/sprecher-blog/cms.git
cd cms
cargo build --release
sudo cp target/release/sprecher /usr/bin/
sprecher
```

It'll serve locally on `http://127.0.0.1:8080`

## Writing an article
Articles are written in a format called sprecher-markdown
It's a markdown format that is built to be used for blogs

```markdown
title: TITLE
author: AUTHOR
description: DESCRIPTION
---
## This is a subheading

- one
- two
- three

| table | thing |
| ----- | ----- |
| left  | right |
```

The information for the article is put at the top and seperated from the body
of the article by a `---` or a `___` (take your pick).

The rest of the body is just markdown. Sprecher will compile it into html on the fly automatically.
