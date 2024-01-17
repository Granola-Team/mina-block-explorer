describe('block spotlight page', () => {
    [{
        origin: '/summary',
        selector: 'a[href^="/blocks/"]:not(a[href^="/blocks/account"])'
    },
    {
        origin: '/blocks',
        selector: 'a[href^="/blocks/"]:not(a[href^="/blocks/account"])'
    },{
        origin: '/accounts/B62qrCz3ehCqi8Pn8y3vWC9zYEB9RKsidauv15DeZxhzkxL3bKeba5h',
        selector: 'a[href^="/blocks/"]:not(a[href^="/blocks/account"])'
    }].forEach(({ origin, selector}) => it(`is navigated to from ${origin}`,() => {
        cy.visit(origin);
        let link = cy.get(selector, {timeout: 10000}).first();
        link.then($a => {
            link.click({force:true});
            cy.url().should('include',`/blocks/${$a.text()}`, {timeout: 10000})
        })
    }));
});