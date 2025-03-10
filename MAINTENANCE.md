# Maintenance guide

## Making a new release

1. Update master branch

   ```shell
   git checkout master && git pull
   ```

1. Update project version in `Cargo.toml`

   ```shell
   vim Cargo.toml
   ```

1. Update `Cargo.lock`

   ```shell
   cargo update
   ```

1. Update `CHANGELOG.md`

   ```shell
   vim CHANGELOG.md
   ```

1. Update pike version in template dependencies

   ```shell
   vim plugin_template/_Cargo.toml
   ```

1. Commit changed files

   ```shell
   git commit -m "chore: bump version" Cargo.toml Cargo.lock CHANGELOG.md plugin_template/_Cargo.toml
   ```

1. Make a new git tag

   ```shell
   git tag -a <NEW_VERSION>
   ```

1. Push all to upstream

   ```shell
   git push origin master --follow-tags
   ```

1. [Create](https://github.com/picodata/pike/releases/new) a new release specifying pushed tag
