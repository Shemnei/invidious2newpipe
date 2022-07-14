# invidious2newpipe

Converts the Invidious export format `Export subscriptions as OPML (for NewPipe & FreeTube)` to NewPipe's import format `Previous export`.

## Usage

1. Export your Invidious subscriptions with the option `Export subscriptions as OPML (for NewPipe & FreeTube)` and save it as `[EXPORTED FILE]`
2. Run `i2n [EXPORTED FILE] [CONVERTED OUTPUT JSON]`
3. Import the `[CONVERTED OUTPUT JSON]` in NewPipe with the option `Import > Previous export`
4. Win

## References

- NewPipe's export/import format: <https://github.com/TeamNewPipe/NewPipe/blob/5be40f62f3eeb52bb43e78db0cb7bcbb1b2be051/app/src/main/java/org/schabi/newpipe/local/subscription/services/ImportExportJsonHelper.java>
