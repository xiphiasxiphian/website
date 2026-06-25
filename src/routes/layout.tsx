import { component$, Slot } from "@builder.io/qwik";
import { Link, useLocation } from "@builder.io/qwik-city";
import "./styles.css";

const NAV_LINKS: { label: string; href: string }[] = [
  { label: "Home", href: "/" },
  { label: "TODO", href: "/about" },
  { label: "TODO", href: "/projects" },
  { label: "TODO", href: "/skills" },
  { label: "TODO", href: "/contact" },
];

export default component$(() => {
  const loc = useLocation();

  return (
    <div class="layout">
      <header class="header">
        <Link href="/" style="text-decoration:none">
          <h1 class="site-title">Siaphix</h1>
        </Link>

        <nav class="nav" aria-label="Main navigation">
          {NAV_LINKS.map(({ label, href }) => (
            <Link
              key={href}
              href={href}
              class="nav-link"
              data-active={loc.url.pathname === href ? "" : undefined}
            >
              {label}
            </Link>
          ))}
        </nav>
      </header>

      <main class="main">
        <Slot />
      </main>

      <footer class="footer">© {new Date().getFullYear()} Siaphix</footer>
    </div>
  );
});
