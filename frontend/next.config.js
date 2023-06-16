/** @type {import('next').NextConfig} */
const nextConfig = {
    "reactStrictMode": false,
    "experimental": {
        "appDir": true,
        "typedRoutes": false,
        "serverActions": true
    },
    "images": {
        "remotePatterns": [
            {   
                "protocol": "https",
                "hostname": "avatars.githubusercontent.com"
            },
            {
                "protocol": "https",
                "hostname": "cdn.discordapp.com"
            },
            {
                "protocol": "https",
                "hostname": "mc-heads.net"
            }
        ]
    }
}

module.exports = nextConfig
