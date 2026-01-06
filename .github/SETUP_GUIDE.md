# GitHub Repository Setup for Docker CI/CD

This guide helps repository administrators configure GitHub to automatically build and publish Docker images.

## ✅ Required Configuration

### Step 1: Enable GitHub Actions

1. Go to your repository on GitHub
2. Click **Settings** (top right)
3. In the left sidebar, click **Actions** → **General**
4. Under "Actions permissions":
   - Select **"Allow all actions and reusable workflows"**
5. Click **Save**

### Step 2: Configure Workflow Permissions

Still in **Settings** → **Actions** → **General**:

1. Scroll down to **"Workflow permissions"**
2. Select **"Read and write permissions"**
3. Check ☑️ **"Allow GitHub Actions to create and approve pull requests"**
4. Click **Save**

### Step 3: Verify Secrets (Optional Check)

1. Go to **Settings** → **Secrets and variables** → **Actions**
2. You should see **`GITHUB_TOKEN`** is automatically available
   - ⚠️ **Note**: `GITHUB_TOKEN` is NOT visible here - it's automatically injected
   - No manual configuration needed!

## ✅ That's It!

**No secrets to create!** The workflow uses the automatic `GITHUB_TOKEN` which GitHub provides to every workflow run.

## Making the Docker Image Public

### Option A: Public Repository
If your repository is public, Docker images are automatically public.

### Option B: Private Repository, Public Image
If your repository is private but you want the Docker image to be public:

1. Push code to trigger the workflow (image gets built)
2. Go to your repository on GitHub
3. Click **"Packages"** in the right sidebar
4. Click on the **`mocked-robot`** package
5. Click **"Package settings"** (gear icon)
6. Scroll to **"Danger Zone"**
7. Click **"Change visibility"**
8. Select **"Public"**
9. Type the repository name to confirm
10. Click **"I understand, change package visibility"**

Now anyone can pull the image without authentication!

## Verifying Everything Works

### Check 1: Workflow is Enabled

1. Go to **Actions** tab
2. You should see "Docker Image" workflow listed
3. If not, check Step 1 above

### Check 2: Trigger a Build

```bash
# Make a small change and push to master
git commit --allow-empty -m "Trigger Docker build"
git push origin master
```

### Check 3: Monitor the Workflow

1. Go to **Actions** tab
2. Click on the latest "Docker Image" workflow run
3. Watch the build progress
4. Should complete in ~2-3 minutes

### Check 4: Verify Image is Published

1. After successful build, go to your repository
2. Look for **"Packages"** in right sidebar (you might need to refresh)
3. Click on **`mocked-robot`**
4. You should see the latest tag (e.g., `latest`, `master`)

### Check 5: Pull and Test the Image

```bash
# Pull the image
docker pull ghcr.io/YOUR_USERNAME/seersdk-rs/mocked-robot:latest

# Run it
docker run -d -p 8080:8080 -p 19204-19210:19204-19210 \
  ghcr.io/YOUR_USERNAME/seersdk-rs/mocked-robot:latest

# Test it
curl http://localhost:8080/waypoints
```

## Troubleshooting

### Problem: Workflow doesn't run

**Solution:**
- Check that Actions are enabled (Step 1)
- Check the workflow file exists: `.github/workflows/docker.yml`
- Check branch name matches (main vs master)

### Problem: "Permission denied" error in workflow

**Solution:**
- Check workflow permissions are set to "Read and write" (Step 2)
- Verify `packages: write` permission in workflow file:
  ```yaml
  permissions:
    contents: read
    packages: write
  ```

### Problem: Image doesn't appear in Packages

**Solution:**
- Wait 2-5 minutes after successful workflow
- Refresh the repository page
- Check workflow logs for errors
- Ensure push was successful in logs:
  ```
  pushing manifest for ghcr.io/user/repo/mocked-robot:latest
  ```

### Problem: Can't pull image (permission denied)

**If repository is private:**
1. Create Personal Access Token (PAT):
   - Go to https://github.com/settings/tokens
   - Click "Generate new token (classic)"
   - Select scope: `read:packages`
   - Copy the token

2. Login to GHCR:
   ```bash
   echo "YOUR_PAT_TOKEN" | docker login ghcr.io -u YOUR_USERNAME --password-stdin
   ```

3. Pull again:
   ```bash
   docker pull ghcr.io/YOUR_USERNAME/seersdk-rs/mocked-robot:latest
   ```

**Or make the image public** (see "Making the Docker Image Public" above)

## Advanced Configuration

### Publishing to Docker Hub (Optional)

If you also want to publish to Docker Hub:

1. Create Docker Hub access token:
   - Go to https://hub.docker.com/settings/security
   - Click "New Access Token"
   - Copy the token

2. Add secrets to GitHub:
   - Go to **Settings** → **Secrets and variables** → **Actions**
   - Click **"New repository secret"**
   - Add `DOCKERHUB_USERNAME` with your Docker Hub username
   - Add `DOCKERHUB_TOKEN` with the access token

3. Update `.github/workflows/docker.yml` to add Docker Hub login

### Customizing Image Names

Edit `.github/workflows/docker.yml`:

```yaml
env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}/mocked-robot  # Change 'mocked-robot' here
```

### Changing Build Platforms

Edit `.github/workflows/docker.yml`:

```yaml
platforms: linux/amd64,linux/arm64  # Add/remove platforms
```

Common platforms:
- `linux/amd64` - x86_64 (Intel/AMD)
- `linux/arm64` - ARM64 (Apple Silicon, AWS Graviton)
- `linux/arm/v7` - ARMv7 (Raspberry Pi)

## Summary Checklist

- ✅ Enable GitHub Actions
- ✅ Set workflow permissions to "Read and write"
- ✅ Push code to trigger build
- ✅ Verify workflow completes successfully
- ✅ Check package appears in sidebar
- ✅ (Optional) Make package public if desired
- ✅ Test pulling and running the image

**That's all you need!** No secrets to configure for GHCR. 🎉

## Reference

- Workflow file: `.github/workflows/docker.yml`
- Documentation: `.github/DOCKER_CI.md`
- Docker files: `docker/`

For detailed CI/CD information, see [DOCKER_CI.md](DOCKER_CI.md).
