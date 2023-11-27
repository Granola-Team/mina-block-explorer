import { component$, useStyles$ } from '@builder.io/qwik';

import styles from './info-section-styles.css?inline';

interface InfoProps {
    label: string
    value: string|number
}

export default component$<InfoProps>((props) => {
    useStyles$(styles);
    return (
        <div class="summary-section__info">
            <label class="summary-section__label" for="totalCurrency">{props.label}:</label>
            <div class="summary-section__data" id="totalCurrency">{props.value}</div>
        </div>
    )
})