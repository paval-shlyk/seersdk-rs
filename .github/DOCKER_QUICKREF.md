# GitHub Actions Docker CI/CD - Quick Reference

## 🔄 Automated Workflow

```
┌─────────────────────────────────────────────────────────────────┐
│                     Developer Actions                             │
└─────────────────────────────────────────────────────────────────┘
                               │
                   ┌───────────┴──────────┐
                   │                       │
              Push to master         Create tag v1.0.0
                   │                       │
                   └───────────┬───────────┘
                               │
┌─────────────────────────────────────────────────────────────────┐
│                    GitHub Actions Workflow                        │
│                  (.github/workflows/docker.yml)                   │
└─────────────────────────────────────────────────────────────────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
         Checkout        Build Docker    Push to GHCR
              │           Image (multi-     (if not PR)
              │           platform)              │
              │                │                │
              └────────────────┴────────────────┘
                               │
┌─────────────────────────────────────────────────────────────────┐
│          GitHub Container Registry (ghcr.io)                     │
│                                                                   │
│  📦 ghcr.io/paval-shlyk/seersdk-rs/mocked-robot                  │
│     ├── latest (from master)                                     │
│     ├── master (from master)                                     │
│     ├── 1.0.0 (from tag v1.0.0)                                 │
│     ├── 1.0 (from tag v1.0.0)                                   │
│     └── 1 (from tag v1.0.0)                                     │
└─────────────────────────────────────────────────────────────────┘
                               │
                               │ docker pull
                               │
┌─────────────────────────────────────────────────────────────────┐
│                          End Users                                │
│                                                                   │
│  docker pull ghcr.io/paval-shlyk/seersdk-rs/mocked-robot:latest │
│  docker run -d -p 8080:8080 ...                                  │
└─────────────────────────────────────────────────────────────────┘
```

## 🔑 Required Secrets: **NONE!**

✅ `GITHUB_TOKEN` is automatically provided by GitHub Actions
- No manual configuration needed
- Automatically has correct permissions
- Automatically expires after workflow

## 📋 Repository Settings Required

### 1. Enable Actions
**Settings → Actions → General**
- ✅ Allow all actions and reusable workflows

### 2. Workflow Permissions
**Settings → Actions → General → Workflow permissions**
- ✅ Read and write permissions
- ✅ Allow GitHub Actions to create and approve pull requests

### 3. That's It!
No additional configuration needed.

## 🏷️ Image Tags

| Trigger | Tags Created |
|---------|--------------|
| `git push origin master` | `latest`, `master` |
| `git push origin main` | `latest`, `main` |
| `git tag v1.2.3 && git push --tags` | `1.2.3`, `1.2`, `1`, `latest` |
| Pull Request | `pr-123` (not pushed) |

## 🖥️ Supported Platforms

- ✅ `linux/amd64` (Intel/AMD x86_64)
- ✅ `linux/arm64` (ARM64/Apple Silicon)

## 📦 Image Details

| Property | Value |
|----------|-------|
| Base Image | `debian:bookworm-slim` |
| Size | ~90 MB |
| User | `robot` (non-root, UID 1000) |
| Health Check | ✅ Enabled |
| Exposed Ports | 19204-19210, 8080 |

## 🚀 Usage Examples

### Pull Latest Image
```bash
docker pull ghcr.io/paval-shlyk/seersdk-rs/mocked-robot:latest
```

### Pull Specific Version
```bash
docker pull ghcr.io/paval-shlyk/seersdk-rs/mocked-robot:1.0.0
```

### Run Container
```bash
docker run -d \
  --name mocked-robot-server \
  -p 19204-19210:19204-19210 \
  -p 8080:8080 \
  ghcr.io/paval-shlyk/seersdk-rs/mocked-robot:latest
```

### Test It Works
```bash
curl http://localhost:8080/waypoints
```

## 🔍 Monitoring Workflow

### View Workflow Runs
1. Go to **Actions** tab
2. Select **"Docker Image"** workflow
3. View latest runs and logs

### Check Published Images
1. Go to repository homepage
2. Click **"Packages"** in right sidebar
3. Click on **`mocked-robot`** package
4. View all available tags

## 📱 Making Image Public

**If repository is private:**

1. Build workflow runs (creates image)
2. Go to **Packages** → **mocked-robot**
3. **Package settings** → **Change visibility**
4. Select **Public**
5. Confirm

Now anyone can pull without authentication!

## ⚡ Build Performance

| Build Type | Time | Cache |
|------------|------|-------|
| First build | ~2-3 min | ❌ |
| Cached build | ~30-60 sec | ✅ |
| Multi-platform | ~3-4 min | ✅ |

## 🐛 Common Issues

### Workflow not running?
- ✅ Check Actions are enabled
- ✅ Check branch name (main vs master)
- ✅ Check workflow file exists

### Permission denied?
- ✅ Check workflow permissions = "Read and write"
- ✅ Check workflow has `packages: write`

### Can't pull image?
- ✅ Repository private? Create PAT and login
- ✅ Or make package public

## 📚 Documentation

- **Detailed CI/CD Guide**: [DOCKER_CI.md](DOCKER_CI.md)
- **Setup Guide**: [SETUP_GUIDE.md](SETUP_GUIDE.md)
- **Workflow File**: [workflows/docker.yml](workflows/docker.yml)
- **Docker Documentation**: [../docker/README.md](../docker/README.md)

## ✨ Key Benefits

✅ **Zero Configuration** - No secrets to manage
✅ **Automatic Builds** - On every push to master
✅ **Multi-Platform** - AMD64 + ARM64
✅ **Cached Builds** - Fast subsequent builds
✅ **Versioned Tags** - Semantic versioning support
✅ **Health Checks** - Built-in container monitoring
✅ **Minimal Size** - Only ~90MB
✅ **Secure** - Non-root user, minimal attack surface
