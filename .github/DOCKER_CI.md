# GitHub CI/CD for Docker Image

This document explains the GitHub Actions workflow for building and publishing the Mocked RBK Robot Docker image.

## Overview

The workflow automatically builds and publishes the Docker image to GitHub Container Registry (GHCR) when:
- Code is pushed to `main` or `master` branch
- A new version tag (e.g., `v1.0.0`) is created
- Pull requests are opened (build only, no push)

## GitHub Secrets Configuration

### Required Secrets

The workflow uses **`GITHUB_TOKEN`** which is **automatically provided by GitHub Actions**. No manual configuration is needed!

### ✅ No Additional Secrets Required!

The `GITHUB_TOKEN` secret is:
- **Automatically created** by GitHub for each workflow run
- **Automatically expires** after the workflow completes
- **Has appropriate permissions** configured in the workflow file
- **No manual setup needed** - it just works!

### Optional Secrets (Only if Publishing to Other Registries)

If you want to publish to Docker Hub or other registries in addition to GHCR, you would need:

1. **For Docker Hub:**
   - `DOCKERHUB_USERNAME` - Your Docker Hub username
   - `DOCKERHUB_TOKEN` - Docker Hub access token (create at https://hub.docker.com/settings/security)

2. **For AWS ECR:**
   - `AWS_ACCESS_KEY_ID` - AWS access key
   - `AWS_SECRET_ACCESS_KEY` - AWS secret key

**But these are NOT needed for the default GHCR setup!**

## Workflow Configuration

### File Location
`.github/workflows/docker.yml`

### Permissions Required

The workflow file already includes the necessary permissions:

```yaml
permissions:
  contents: read    # Read repository contents
  packages: write   # Write to GitHub Packages/Container Registry
```

### Image Tagging Strategy

The workflow automatically creates the following tags:

| Event | Tags Created | Example |
|-------|-------------|---------|
| Push to master | `latest`, `master` | `ghcr.io/owner/repo/mocked-robot:latest` |
| Push to main | `latest`, `main` | `ghcr.io/owner/repo/mocked-robot:latest` |
| Version tag | `v1.2.3`, `1.2.3`, `1.2`, `1`, `latest` | `ghcr.io/owner/repo/mocked-robot:1.2.3` |
| Pull request | `pr-123` | `ghcr.io/owner/repo/mocked-robot:pr-123` (not pushed) |

### Multi-Platform Builds

The workflow builds images for both:
- `linux/amd64` (x86_64)
- `linux/arm64` (ARM64/Apple Silicon)

## Using the Published Image

### Public Repository

If your repository is **public**, the image is automatically public and can be pulled by anyone:

```bash
# Pull latest
docker pull ghcr.io/paval-shlyk/seersdk-rs/mocked-robot:latest

# Pull specific version
docker pull ghcr.io/paval-shlyk/seersdk-rs/mocked-robot:1.0.0

# Run the image
docker run -d -p 19204-19210:19204-19210 -p 8080:8080 \
  ghcr.io/paval-shlyk/seersdk-rs/mocked-robot:latest
```

### Private Repository

If your repository is **private**, you need to authenticate:

1. **Create a Personal Access Token (PAT):**
   - Go to https://github.com/settings/tokens
   - Click "Generate new token" → "Generate new token (classic)"
   - Select scopes: `read:packages`
   - Copy the token (you won't see it again!)

2. **Login to GHCR:**
   ```bash
   echo "YOUR_PAT_TOKEN" | docker login ghcr.io -u YOUR_GITHUB_USERNAME --password-stdin
   ```

3. **Pull the image:**
   ```bash
   docker pull ghcr.io/paval-shlyk/seersdk-rs/mocked-robot:latest
   ```

## Making the Image Public

Even if your repository is private, you can make the Docker image public:

1. Go to your repository on GitHub
2. Click on "Packages" in the right sidebar
3. Click on the `mocked-robot` package
4. Go to "Package settings"
5. Scroll down to "Danger Zone"
6. Click "Change visibility" → Select "Public"

Now anyone can pull the image without authentication!

## Workflow Triggers

### Automatic Triggers

| Trigger | Action | Push Image? |
|---------|--------|-------------|
| Push to master | Build multi-platform image | ✅ Yes |
| Push to main | Build multi-platform image | ✅ Yes |
| Create tag `v*` | Build multi-platform image | ✅ Yes |
| Pull request | Build for testing only | ❌ No |

### Manual Trigger

You can also manually trigger the workflow:

1. Go to Actions tab in your repository
2. Select "Docker Image" workflow
3. Click "Run workflow"
4. Select branch and click "Run workflow"

## Viewing Published Images

### On GitHub

1. Go to your repository
2. Click "Packages" in the right sidebar
3. You'll see the `mocked-robot` package
4. Click it to see all available tags and details

### Using GitHub CLI

```bash
# List packages
gh api user/packages/container/mocked-robot/versions

# View package details
gh api /user/packages/container/mocked-robot
```

## Build Cache

The workflow uses GitHub Actions cache to speed up builds:
- First build: ~2-3 minutes
- Subsequent builds: ~30-60 seconds (if no major changes)

## Troubleshooting

### "Permission denied" when pushing image

**Solution:** Ensure the workflow has `packages: write` permission:
```yaml
permissions:
  packages: write
```

### "Failed to login to ghcr.io"

**Solution:** Check that `GITHUB_TOKEN` has the correct permissions. The token is automatically provided, but repository settings might restrict it.

1. Go to repository Settings → Actions → General
2. Scroll to "Workflow permissions"
3. Select "Read and write permissions"
4. Save

### Build fails during Rust compilation

**Solution:** The multi-stage build uses more memory for multi-platform builds. GitHub Actions runners should handle it, but if it fails:
- Remove `linux/arm64` from platforms temporarily
- Or add platform-specific caching

### Image not appearing in Packages

**Solution:** 
1. Check the workflow run logs in Actions tab
2. Verify the image was pushed (not a PR build)
3. Wait a few minutes - it might take time to appear

## Example: Complete CI/CD Flow

1. **Developer pushes to master:**
   ```bash
   git add .
   git commit -m "Update mock server"
   git push origin master
   ```

2. **GitHub Actions automatically:**
   - Runs tests (ci.yml)
   - Builds Docker image for amd64 and arm64
   - Pushes to `ghcr.io/owner/repo/mocked-robot:latest`
   - Tags as `master` as well

3. **Users can immediately pull:**
   ```bash
   docker pull ghcr.io/paval-shlyk/seersdk-rs/mocked-robot:latest
   docker run -d -p 19204-19210:19204-19210 -p 8080:8080 \
     ghcr.io/paval-shlyk/seersdk-rs/mocked-robot:latest
   ```

4. **For releases:**
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
   
   Creates images:
   - `ghcr.io/owner/repo/mocked-robot:1.0.0`
   - `ghcr.io/owner/repo/mocked-robot:1.0`
   - `ghcr.io/owner/repo/mocked-robot:1`
   - `ghcr.io/owner/repo/mocked-robot:latest`

## Repository Settings Checklist

✅ **Enable GitHub Actions:**
- Settings → Actions → General → "Allow all actions"

✅ **Enable Workflow Permissions:**
- Settings → Actions → General → Workflow permissions
- Select "Read and write permissions"
- Check "Allow GitHub Actions to create and approve pull requests"

✅ **Enable Packages:**
- Packages are enabled by default for all repositories

That's it! No secrets to configure - it just works! 🚀

## Additional Resources

- [GitHub Container Registry Documentation](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry)
- [GitHub Actions Docker Build Guide](https://docs.github.com/en/actions/publishing-packages/publishing-docker-images)
- [docker/build-push-action Documentation](https://github.com/docker/build-push-action)
