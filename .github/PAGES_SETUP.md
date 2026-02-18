# GitHub Pages Setup

This repository uses GitHub Pages to automatically publish Rust API documentation.

## Enabling GitHub Pages

To enable automatic documentation publishing:

1. Go to your repository on GitHub
2. Click on **Settings** tab
3. In the left sidebar, click **Pages**
4. Under **Source**, select **GitHub Actions**
5. Save the changes

![Pages Settings](https://docs.github.com/assets/cb-47267/mw-1440/images/help/pages/pages-source-github-actions.webp)

## What Gets Published

The `docs.yml` workflow automatically:
- Builds Rust API documentation with `cargo doc`
- Deploys it to GitHub Pages at `https://npequeux.github.io/rutree2/`
- Runs on every push to `main` and every tag push

## Troubleshooting

### Error: "Not Found" when accessing configure-pages

**Solution:** GitHub Pages is not enabled. Follow the steps above to enable it.

### Workflow fails with "Get Pages site failed"

**Solution:** Make sure you've selected **GitHub Actions** as the source in Pages settings, not "Deploy from a branch".

### Documentation builds but doesn't deploy

**Possible causes:**
1. Pages not enabled in Settings
2. Another workflow is using the same deployment concurrency group
3. Insufficient permissions (the workflow has `pages: write` permission)

## Documentation URL

Once enabled, your documentation will be available at:
- **Main site:** https://npequeux.github.io/rutree2/
- **API docs:** https://npequeux.github.io/rutree2/rutree2/

## Testing Locally

To build and view documentation locally:

```bash
cargo doc --no-deps --open
```

This opens the documentation in your browser without deploying to GitHub Pages.
