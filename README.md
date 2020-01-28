# react-scaffold-component
> Easy scaffolding of React SFCs.

RSC can easily create your stateless function component.

Supported:

- [ ] Plain SFC
- [ ] Styled (via `styled-component`)
- [ ] Routed (via `react-router-dom`)

Ideally, all options should allow for arbitrary combination.

For example, sfc+styled+routed:
```jsx
import {React} from 'react';
import styled from 'styled-component';
import {
    Link,
    Switch,
    useRouteMatch,
} from 'react-router-dom';

const MyComponent = ({className}) => {
    const {path} = useRouteMatch();

    return (
        <div className={className}>
            {/* Your code goes here */}
        </div>
    );
};

export default styled(MyComponent)`
    /* Your style goes here */
`;
```
