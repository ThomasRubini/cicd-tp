{{- define "mychart.is_dev_env" -}}
{{ eq .Values.image.tag "latest" }}
{{- end -}}
