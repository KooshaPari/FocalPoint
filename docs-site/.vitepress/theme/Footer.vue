<template>
  <footer class="vp-footer-custom">
    <div class="footer-container">
      <div class="footer-content">
        <div class="footer-section">
          <p class="footer-copyright">
            © 2026 FocalPoint · Released under MIT OR Apache-2.0
          </p>
        </div>
        <nav class="footer-links">
          <a href="https://github.com/KooshaPari/FocalPoint" target="_blank" rel="noopener">
            GitHub
          </a>
          <a href="https://discord.gg/focalpoint" target="_blank" rel="noopener">
            Discord
          </a>
          <a href="/governance/coc" title="Code of Conduct">
            Code of Conduct
          </a>
          <a href="/governance/contributing" title="Contributing Guidelines">
            Contributing
          </a>
        </nav>
      </div>
    </div>
  </footer>
</template>

<style scoped>
.vp-footer-custom {
  border-top: 1px solid var(--vp-c-divider);
  background-color: var(--vp-c-bg-soft);
  padding: 2rem 1.5rem;
  margin-top: 3rem;
  font-size: 0.875rem;
  color: var(--vp-c-text-2);
}

.footer-container {
  max-width: 1152px;
  margin: 0 auto;
}

.footer-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.footer-section {
  margin: 0;
}

.footer-copyright {
  margin: 0;
  color: var(--vp-c-text-3);
}

.footer-links {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5rem;
  list-style: none;
  padding: 0;
  margin: 0;
}

.footer-links a {
  color: var(--vp-c-text-2);
  text-decoration: none;
  transition: color 0.2s ease;
}

.footer-links a:hover {
  color: var(--vp-c-brand-1);
  text-decoration: underline;
}

@media (min-width: 768px) {
  .footer-content {
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
  }
}
</style>
