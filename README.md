# stateless-function-component
> Easy scaffolding of stateless function components.

Supported:

- [x] Plain SFC
- [x] Styled (via `styled-component`)
- [x] Routed (via `react-router-dom`)

All options can be mixed and combined.

Usage:

```bash
# show all options
sfc --help

# scaffold styled sfc with routing
sfc styled routed
```

Example output for `sfc styled routed`:
```jsx
import {React} from 'react';
import styled from 'styled-components';
import {
    Link,
    Switch,
    useRouteMatch,
} from 'react-router-dom';

const MyComponent = ({className}) => {
    const {path} = useRouteMatch();

    return (
        <div className={className}>
        </div>
    );
};

export default styled(MyComponent)``;
```
