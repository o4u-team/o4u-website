import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import '@mdi/font/css/materialdesignicons.css'

export default createVuetify({
    components,
    directives,
    theme: {
        defaultTheme: 'light',
        themes: {
            light: {
                colors: {
                    primary: '#2E7D32',
                    secondary: '#1B5E20',
                    accent: '#66BB6A',
                    error: '#FF5252',
                    info: '#0288D1',
                    success: '#43A047',
                    warning: '#F9A825',
                },
            },
        },
    },
})
