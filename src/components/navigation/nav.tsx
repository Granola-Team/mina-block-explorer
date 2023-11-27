import { component$, useStyles$ } from "@builder.io/qwik";
import { Link } from "@builder.io/qwik-city";

import styles from './styles.css?inline';

export default component$(() => {
    useStyles$(styles)
    return (
        <nav>
            <Link class="nav__link" href="/summary">Summary</Link>
            <Link class="nav__link" href="/accounts">Accounts</Link>
            <Link class="nav__link" href="/blocks">Blocks</Link>
        </nav>
    );
});