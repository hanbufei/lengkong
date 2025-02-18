import { render } from 'solid-js/web'
import './index.css'
import { Router } from '@solidjs/router';
import Login from './pages/login';
import Admin from './pages/admin';
import New from './pages/new';

const routes = [
    {
        path: "/",
        component: () => <Login />,
    },
    {
        path: "/admin",
        component: () => <Admin/>
    },
    {
        path: "/new",
        component: () => <New/>,
    }
]


render(() => (
    <Router>{routes}</Router>
    ), 
document.getElementById('root'));


