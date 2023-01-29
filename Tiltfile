
#k8s_yaml(local('helm template --set key1=val1,key2=val2 ./charts/main-chart'))
#watch_file('/charts/main-chart')

local_resource('cargo', cmd='cargo', labels=['cargo'])
local_resource('go', cmd='go mod tidy', labels=['go'])
local_resource('cargo-list', cmd='cargo --list', deps=['Cargo.toml'], labels=['cargo'])
local_resource('pnpm', cmd='pnpm install', deps=['package.json', 'pnpm-lock.yaml'], labels=['pnpm'])
local_resource('bazel-coverage-go-app', cmd='bazel coverage //apps/go/go-app', labels=['bazel'])
local_resource('bazel-query-go-app', cmd='bazel query "deps(//apps/go/go-app:*)"', labels=['bazel'])

# local_resource('bazel-run-go-app', trigger_mode=TRIGGER_MODE_MANUAL, cmd='bazel run //apps/go/go-app', labels=['bazel'])
local_resource('bazel-query-foo', cmd='bazel query //:foo --output=build', labels=['bazel'], deps=['apps/go/go-app/BUILD.bazel', "WORKSPACE.bazel", "BUILD.bazel"])
local_resource('bazel-gazelle', cmd='bazel run //:gazelle', labels=['bazel'], deps=['.'])
# local_resource('bazel-modquery', cmd='bazel modquery', labels=['bazel'])
local_resource('bazel-build-go-app', cmd='bazel build //apps/go/go-app:BUILD.bazel', labels=['bazel'], deps=['apps/go/go-app/BUILD.bazel', "WORKSPACE.bazel", "BUILD.bazel"])
local_resource('bazel-clean', trigger_mode=TRIGGER_MODE_MANUAL, cmd='bazel clean', labels=['bazel'], deps=['apps/go/go-app/BUILD.bazel', "WORKSPACE.bazel", "BUILD.bazel"])
local_resource('bazel-query1-go-app', cmd='bazel query //apps/go/go-app:BUILD.bazel --output=build', deps=['.'], labels=['bazel'])
local_resource('bazel-version', cmd='bazel version', labels=['bazel'], deps=['apps/go/go-app/BUILD.bazel', "WORKSPACE.bazel", "BUILD.bazel"])
local_resource('bazel-info', cmd='bazel info', labels=['bazel'], deps=['apps/go/go-app/BUILD.bazel', "WORKSPACE.bazel", "BUILD.bazel"])
local_resource('bazel-test', cmd='bazel test //:all', labels=['bazel'], deps=['apps/go/go-app/BUILD.bazel', "WORKSPACE.bazel", "BUILD.bazel"])
local_resource('bazel-build', cmd='bazel build //:all', labels=['bazel'], deps=['apps/go/go-app/BUILD.bazel', "WORKSPACE.bazel", "BUILD.bazel"])
local_resource('bazel-query', cmd='bazel query "//:*"', labels=['bazel'], deps=['apps/go/go-app/BUILD.bazel', "WORKSPACE.bazel", "BUILD.bazel"])
local_resource('bazel-cquery', cmd='bazel cquery //:foo', labels=['bazel'], deps=['apps/go/go-app/BUILD.bazel', "WORKSPACE.bazel", "BUILD.bazel"])
# local_resource('bazel-query', cmd='bazel query //:foo', labels=['bazel'])
# local_resource('bazel-coverage', cmd='bazel coverage "//..."', deps=['.'], labels=['bazel'])
# local_resource('bazel-analyze', cmd='bazel analyze-profile', deps=['.'], labels=['just'])
local_resource('bazel-mobile-install', cmd='bazel mobile-install', labels=['bazel'])
# local_resource('bazel-test', cmd='bazel test //...', deps=['.'], labels=['just'])
# local_resource('bazel-clean', cmd='bazel clean', deps=['.'], labels=['just'])
# local_resource('bazel-coverage', cmd='bazel coverage', deps=['.'], labels=['just'])
# local_resource('proto-generate', cmd='just proto-generate', deps=['_proto/'], labels=['just'])
# local_resource('bazel-print_action', cmd='bazel print_action', deps=['.'], labels=['just'])
# include('./k8s/helm/Tiltfile')
# local_resource('tilt-ci', cmd='tilt ci', deps=['.'], labels=['tilt'])

# include('./apps/go/api-rest/Tiltfile')
# include('./apps/node/api-rest/Tiltfile')
# include('./apps/frontend/host/Tiltfile')
# include('./apps/node/users-grpc/Tiltfile')
# include('./apps/rust/api-rest/Tiltfile')
# include('./apps/rust/users-grpc/Tiltfile')
# include('./apps/infra/commdands/Tiltfile')

# k8s_yaml(kustomize('k8s/base'))
# k8s_resource("go-api-rest", port_forwards="5001:8080")

load('ext://uibutton', 'cmd_button', 'location', 'text_input', 'bool_input')


cmd_button(name='NX',
        argv=['sh', '-c','pnpm nx $type --parallel --max-parallel=$cores $SKIP_CASHE --target=$TARGET'],
        text='NX',
        location=location.NAV,
        requires_confirmation=True,
        inputs=[
            text_input('type', placeholder='Enter your nx command type', default="affected"),
            text_input('TARGET', placeholder='Enter your nx command target', default="test"),
            bool_input('SKIP_CASHE', true_string='--skip-nx-cache', false_string=''),
            text_input('cores', placeholder='Enter value or --max-parallel', default="2"),
        ],
        icon_name='travel_explore')
    
cmd_button(name='Graph',
        argv=['sh', '-c','pnpm nx affected:dep-graph'],
        text='Graph',
        location=location.NAV,
        inputs=[
            bool_input('AFFECTED', true_string='affected:', false_string=''),
        ],
        icon_name='grain')

# cmd_button(name='bazel-build',
#     argv=['sh', '-c','bazel build //...'],
#     text='NX',
#     location=location.NAV,
#     requires_confirmation=False,
#     icon_name='travel_explore')
    # inputs=[
    # text_input('type', placeholder='Enter your nx command type', default="affected"),
    # text_input('TARGET', placeholder='Enter your nx command target', default="test"),
    # bool_input('SKIP_CASHE', true_string='--skip-nx-cache', false_string=''),
    # text_input('cores', placeholder='Enter value or --max-parallel', default="2"),
    # ],
    