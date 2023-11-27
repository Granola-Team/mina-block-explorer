import { createDOM } from '@builder.io/qwik/testing';
import { test, expect } from 'vitest';
import InfoSection from './info-section';

 
test(`[InfoSection Component]: Should render label and value`, async () => {
  const { screen, render } = await createDOM();
  await render(<InfoSection label="test" value={100} />);
  expect(screen.outerHTML).toContain("test");
  expect(screen.outerHTML).toContain("100");
});
