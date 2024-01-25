describe('block spotlight', () => {
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
        cy.wait(500);
        cy.get(selector, {timeout: 10000}).first().click({force:true});
        cy.wait(500);
        cy.url().should('include','/blocks/', {timeout: 10000})
    }));
});